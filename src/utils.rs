use crate::cli::Cli;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

pub fn get_current_version_from_config(config_content: &str) -> Option<String> {
    let current_version_regex =
        Regex::new(r#"\[bumpversion\]\s*current_version\s*=\s*(?P<version>\d+(\.\d+){0,2})\s*"#)
            .unwrap();

    if let Some(captures) = current_version_regex.captures(config_content) {
        if let Some(version) = captures.name("version") {
            return Some(version.as_str().to_string());
        }
    }

    None
}

// Function to read files from the configuration file
pub fn read_files_from_config(config_file: &str) -> Result<HashSet<String>, std::io::Error> {
    let config_content = fs::read_to_string(config_file)?;
    let mut config_files = HashSet::new();

    for line in config_content.lines() {
        if let Some(file_section) = line.strip_prefix("[bumpversion:file:") {
            if let Some(file_name) = file_section.split(']').next() {
                config_files.insert(file_name.trim().to_string());
            }
        }
    }

    Ok(config_files)
}

pub fn attempt_version_bump(args: Cli) -> Option<String> {
    let parse_regex = args.parse.clone();
    let regex = match Regex::new(&parse_regex) {
        Ok(r) => r,
        Err(_) => {
            eprintln!("--patch '{}' is not a valid regex", args.parse.clone());
            return None;
        }
    };

    let config_content = fs::read_to_string(args.config_file.clone()).unwrap();
    let current_version = get_current_version_from_config(&config_content).unwrap_or_else(|| {
        panic!("Failed to extract current version from config file");
    });
    // let current_version = args.current_version.unwrap_or("".to_string());
    let mut parsed: HashMap<String, String> = HashMap::new();

    if let Some(captures) = regex.captures(&current_version) {
        for name in regex.capture_names() {
            if let Some(name) = name {
                if let Some(capture) = captures.name(name) {
                    parsed.insert(name.to_string(), capture.as_str().to_string());
                }
            }
        }
    }

    let order: Vec<&str> = args
        .serialize
        .match_indices('{')
        .map(|(i, _)| args.serialize[i + 1..].split('}').next().unwrap().trim())
        .collect();

    let mut bumped = false;

    for label in order {
        if let Some(part) = parsed.get_mut(label) {
            if label == args.bump {
                if let Ok(new_value) = part.parse::<u64>() {
                    *part = (new_value + 1).to_string();
                    bumped = true;
                } else {
                    eprintln!("Failed to parse '{}' as u64", part);
                    return None;
                }
            } else if bumped {
                *part = "0".to_string();
            }
        }
    }

    if bumped {
        let new_version = format!(
            "{}.{}.{}",
            parsed.get("major").unwrap_or(&"0".to_string()),
            parsed.get("minor").unwrap_or(&"0".to_string()),
            parsed.get("patch").unwrap_or(&"0".to_string())
        );
        let version = args
            .serialize
            .replace("{major}.{minor}.{patch}", &new_version);
        Some(version)
    } else {
        None
    }
}
