// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use bump2version::cli::Cli;
use bump2version::error::BumpError;
use bump2version::files::{apply_config_version_update, apply_file_change};
use bump2version::git::{assert_clean_working_tree, commit_files, create_tag, get_git_author};
use bump2version::utils::{collect_file_configs, compute_new_version, load_config};
use clap::Parser;
use std::fs;

fn main() -> Result<(), BumpError> {
    let args = Cli::parse();

    let cfg = load_config(&args.config_file, Some(&args.parse), Some(&args.serialize))?;

    let current_version = args
        .current_version
        .clone()
        .or_else(|| cfg.current_version.clone())
        .ok_or_else(|| {
            BumpError::InvalidConfig(
                "current_version not set in config or --current-version flag".into(),
            )
        })?;

    let new_version = args
        .new_version
        .clone()
        .or_else(|| {
            compute_new_version(
                &args.config_file,
                &args.bump,
                Some(&current_version),
                Some(&args.parse),
                Some(&args.serialize),
            )
        })
        .ok_or_else(|| BumpError::InvalidConfig("Could not compute new version".into()))?;

    let dry_run = args.dry_run;
    let do_commit = args.commit;
    let do_tag = args.tag;

    let message = args
        .message
        .replace("{current_version}", &current_version)
        .replace("{new_version}", &new_version);

    let tag_name = cfg
        .tag_name
        .replace("{new_version}", &new_version)
        .replace("{current_version}", &current_version);

    let current_dir = std::env::current_dir()?;
    let repo = gix::open(current_dir.to_str().unwrap())
        .map_err(|e| BumpError::GitError("open_repo".into(), e.to_string()))?;

    if !cfg.allow_dirty {
        assert_clean_working_tree(&repo)?;
    }

    let file_configs = collect_file_configs(&cfg, &args.files);
    let mut changed_paths: Vec<String> = Vec::new();

    for fc in &file_configs {
        let content = fs::read_to_string(&fc.path)?;
        let updated = apply_file_change(&content, fc, &cfg, &current_version, &new_version)?;
        if !dry_run {
            fs::write(&fc.path, &updated)?;
        }
        changed_paths.push(fc.path.clone());
    }

    if fs::metadata(&args.config_file).is_ok() {
        let config_content = fs::read_to_string(&args.config_file)?;
        let updated_config =
            apply_config_version_update(&config_content, &current_version, &new_version);
        if !dry_run {
            fs::write(&args.config_file, &updated_config)?;
            changed_paths.push(args.config_file.clone());
        }
    }

    if do_commit && !dry_run {
        let (author_name, author_email) = get_git_author(&repo)?;

        let commit_id = commit_files(&repo, &changed_paths, &message, &author_name, &author_email)?;

        println!("Committed: {commit_id}");

        if do_tag {
            create_tag(&repo, &tag_name, commit_id)?;
            println!("Git lightweight tag created: refs/tags/{tag_name}");
        }
    } else if do_commit {
        println!(
            "[dry-run] Would commit {} file(s) with message: {}",
            changed_paths.len(),
            message
        );
    }

    Ok(())
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
