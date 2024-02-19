use self::cli::Cli;
use crate::utils::attempt_version_bump;
use clap::Parser;
use std::fs;
use std::process::Command;

mod cli;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args = Cli::parse();
    let config_file = args.config_file.clone();
    let current_version = args.current_version.clone();

    let attempted_new_version = attempt_version_bump(args.clone());

    if !args.new_version.is_empty() && attempted_new_version.is_some() {
        // TODO: fix let new_version = attempted_new_version.clone().unwrap();
        let new_version = args.new_version.clone();

        let dry_run = args.dry_run;
        let commit = args.commit;
        let tag = args.tag;
        let message = args.message;

        let files: Vec<String> = args.files;

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
                &format!("new_version={}", attempted_new_version.unwrap()),
                "",
            );
            config_content = config_content.replace(
                &format!("current_version={}", current_version),
                &format!("current_version={}", new_version),
            );

            if !dry_run {
                fs::write(config_file.clone(), config_content)?;
                commit_files.push(config_file);
            }
        }

        // Git commit and tag
        if commit {
            for path in &commit_files {
                Command::new("git").arg("add").arg(path).output()?;
            }

            Command::new("git")
                .arg("commit")
                .arg("-m")
                .arg(
                    message
                        .replace("{current_version}", &current_version)
                        .replace("{new_version}", &new_version),
                )
                .output()?;

            if tag {
                Command::new("git")
                    .arg("tag")
                    .arg(format!("v{}", new_version))
                    .output()?;
            }
        }
    } else {
        println!("No files specified");
    }

    Ok(())
}
