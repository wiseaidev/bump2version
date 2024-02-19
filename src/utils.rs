use crate::cli::Cli;
use regex::Regex;
use std::collections::HashMap;

pub fn attempt_version_bump(args: Cli) -> Option<String> {
    let parse_regex = args.parse;
    let regex = Regex::new(&parse_regex).ok()?;

    let current_version = args.current_version;
    let match_result = regex.captures_iter(&current_version);

    let mut parsed: HashMap<&str, &str> = HashMap::new();

    for caps in match_result {
        if let (Some(name), Some(value)) = (caps.name("name"), caps.name("value")) {
            parsed.insert(name.as_str(), value.as_str());
        }
    }

    let order: Vec<&str> = args
        .serialize
        .match_indices('{')
        .map(|(i, _)| args.serialize[i + 1..].split('}').next().unwrap())
        .map(|s| s.trim())
        .collect();

    let mut bumped = true;
    for label in order {
        if label == args.bump {
            if let Some(_part) = parsed.get_mut(label) {
                // TODO: fix
                // let new_value = part.parse::<u64>().unwrap() + 1;
                // *part = &new_value.clone().to_string();
                bumped = true;
            }
        } else if bumped {
            parsed.insert(label, "0");
        }
    }

    if bumped {
        let new_version = args.serialize.replace(
            |c| c == '{' || c == '}',
            parsed.get(&"{").unwrap_or(&"").to_string().as_str(), // TODO: fix c
        );
        Some(new_version)
    } else {
        None
    }
}
