use self::cli::Cli;
use crate::utils::attempt_version_bump;
use crate::utils::get_current_version_from_config;
use crate::utils::read_files_from_config;
use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::process::Command;

mod cli;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let config_file = args.config_file.clone();
    let config_content = fs::read_to_string(args.config_file.clone()).unwrap();
    let config_version = get_current_version_from_config(&config_content).ok_or("")?;
    let current_version = args
        .current_version
        .clone()
        .unwrap_or(config_version)
        .clone();

    let attempted_new_version = attempt_version_bump(args.clone());

    if attempted_new_version.is_some() {
        let new_version = attempted_new_version.clone().unwrap();

        let dry_run = args.dry_run;
        let commit = args.commit;
        let tag = args.tag;
        let message = args.message;

        let files: Vec<String> = if args.files.is_empty() {
            let config_files: HashSet<String> = read_files_from_config(&args.config_file)?;
            config_files.into_iter().collect()
        } else {
            args.files
        };

        // Check if Git working directory is clean
        if fs::metadata(".git").is_ok() {
            let git_status = Command::new("git")
                .arg("status")
                .arg("--porcelain")
                .output()?;

            let git_output = String::from_utf8_lossy(&git_status.stdout);
            let git_lines: Vec<&str> = git_output
                .lines()
                .filter(|line| !line.trim().starts_with("??"))
                .collect();

            if !git_lines.is_empty() {
                panic!("Git working directory not clean:\n{}", git_lines.join("\n"));
            }
        }

        // Update version in specified files
        for path in &files {
            let content = fs::read_to_string(path)?;

            if !content.contains(&current_version) {
                panic!("Did not find string {} in file {}", current_version, path);
            }

            let updated_content = content.replace(&current_version, &new_version);

            if !dry_run {
                fs::write(path, updated_content)?;
            }
        }

        let mut commit_files = files.clone();

        // Update config file if applicable
        if fs::metadata(config_file.clone()).is_ok() {
            let mut config_content = fs::read_to_string(config_file.clone())?;

            config_content = config_content.replace(
                &format!("current_version = {}", current_version),
                &format!("current_version = {}", new_version),
            );

            if !dry_run {
                fs::write(config_file.clone(), config_content)?;
                commit_files.push(config_file);
            }
        }
        if commit {
            for path in &commit_files {
                let git_add_output = Command::new("git").arg("add").arg(path).output();

                match git_add_output {
                    Ok(output) => {
                        if !output.status.success() {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            eprintln!("Error during git add:\n{}", stderr);
                        }
                    }
                    Err(err) => {
                        eprintln!("Failed to execute git add: {}", err);
                    }
                }
            }
            let git_diff_output = Command::new("git").arg("diff").output();

            match git_diff_output {
                Ok(output) => {
                    if output.stdout.is_empty() {
                        let commit_output = Command::new("git")
                            .arg("commit")
                            .arg("-m")
                            .arg(
                                message
                                    .replace("{current_version}", &current_version)
                                    .replace("{new_version}", &new_version),
                            )
                            .output();

                        match commit_output {
                            Ok(commit_output) => {
                                if commit_output.status.success() {
                                    println!("Git commit successful");
                                } else {
                                    eprintln!(
                                        "Error during git commit:\n{}",
                                        String::from_utf8_lossy(&commit_output.stderr)
                                    );
                                }
                            }
                            Err(err) => {
                                eprintln!("Failed to execute git commit: {}", err);
                            }
                        }
                    } else {
                        println!("No changes to commit. Working tree clean.");
                    }
                }
                Err(err) => {
                    eprintln!("Failed to execute git diff: {}", err);
                }
            }

            if tag {
                Command::new("git")
                    .arg("tag")
                    .arg(format!("v{}", new_version))
                    .output()?;
            }
        }
    } else {
        eprintln!("No files specified");
    }

    Ok(())
}
