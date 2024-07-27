use clap::{value_parser, Arg, ArgAction, Command};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_yaml::Value as YamlValue;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize)]
struct Frontmatter {
    #[serde(flatten)]
    data: BTreeMap<String, YamlValue>,
    file: String,
}

fn process_file(path: &Path) -> Option<Frontmatter> {
    let file = File::open(path).ok()?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer).ok()?;
    let contents = String::from_utf8_lossy(&buffer);

    let mut lines = contents.lines();

    let yaml_start = lines.position(|line| line == "---")?;
    let yaml_end = lines
        .clone()
        .skip(yaml_start + 3)
        .position(|line| line == "---")
        .map(|pos| pos + yaml_start + 6)
        .unwrap_or(contents.len());

    let yaml_str = lines
        .skip(yaml_start + 3)
        .take(yaml_end - yaml_start - 6)
        .fold(String::new(), |acc, line| acc + line + "\n");

    let yaml_value: YamlValue = serde_yaml::from_str(&yaml_str).ok()?;
    let data = yaml_value
        .as_mapping()
        .map(|mapping| {
            mapping
                .iter()
                .map(|(k, v)| (k.as_str().unwrap_or("").to_string(), v.clone()))
                .collect()
        })
        .unwrap_or_default();
    let file = path.to_string_lossy().to_string();

    Some(Frontmatter { data, file })
}

fn main() {
    let matches = Command::new("frust")
        .version("0.1.0")
        .about("A tool to extract frontmatter from markdown files")
        .arg(
            Arg::new("input")
                .help("Input file or directory")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("output")
                .help("Output file")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("recursive")
                .help("Recursively process directories")
                .short('R')
                .long("recursive")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .help("Verbose output")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let input = matches.get_one::<PathBuf>("input").unwrap();
    let output = matches.get_one::<PathBuf>("output");
    let recursive = matches.get_flag("recursive");
    let verbose = matches.get_flag("verbose");

    if input.is_dir() {
        if !recursive {
            eprintln!("Input is a directory, but --recursive was not specified");
            std::process::exit(1);
        }
    } else if input.extension().map_or(true, |e| e != "md") {
        eprintln!("File must have a .md extension");
        std::process::exit(1);
    }

    let frontmatters: Vec<_> = WalkDir::new(input)
        .into_iter()
        .par_bridge()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |e| e == "md"))
        .filter_map(|entry| process_file(&entry.path()))
        .collect();

    if let Some(output) = output {
        let mut file = BufWriter::new(File::create(output).unwrap());
        for frontmatter in &frontmatters {
            let json = serde_json::to_string_pretty(&frontmatter).unwrap();
            file.write_all(json.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
    } else {
        for frontmatter in &frontmatters {
            let json = serde_json::to_string_pretty(&frontmatter).unwrap();
            println!("{}", json);
        }
    }

    if verbose {
        println!("Processed {} files", frontmatters.len());
    }
}

