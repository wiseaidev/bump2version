use std::{fs, os::unix::fs::PermissionsExt, path::Path};

use anyhow::anyhow;
use clap::Parser;
use gix::bstr::BString;
use gix::object::tree::EntryKind;
use gix::objs;
use gix::{open as open_repo, progress::Discard};
use smallvec::SmallVec;

use self::cli::Cli;
use crate::utils::{attempt_version_bump, get_current_version_from_config, read_files_from_config};

mod cli;

mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let config_file = args.config_file.clone();
    let config_content = fs::read_to_string(&config_file)?;
    let config_version = get_current_version_from_config(&config_content)
        .ok_or("failed to get current version from config")?;

    let current_version = args
        .current_version
        .clone()
        .unwrap_or(config_version)
        .clone();

    let attempted_new_version = args
        .new_version
        .clone()
        .or_else(|| attempt_version_bump(args.clone()));

    if let Some(new_version) = attempted_new_version {
        let dry_run = args.dry_run;
        let commit = args.commit;
        let tag = args.tag;
        let message = args.message;

        let files: Vec<String> = if args.files.is_empty() {
            read_files_from_config(&args.config_file)?
                .into_iter()
                .collect()
        } else {
            args.files
        };
        let repo = open_repo(".")?;

        let statuses = repo.status(Discard)?;
        let mut changes = statuses.into_iter(Vec::<BString>::new())?;
        if changes.next().is_some() {
            panic!("Git working directory not clean.");
        }

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

        if fs::metadata(&config_file).is_ok() {
            let mut updated_config = fs::read_to_string(&config_file)?;
            updated_config = updated_config.replace(
                &format!("current_version = {}", current_version),
                &format!("current_version = {}", new_version),
            );

            if !dry_run {
                fs::write(&config_file, updated_config)?;
                commit_files.push(config_file.clone());
            }
        }
        if commit {
            let mut entries = Vec::new();

            for path_str in &commit_files {
                let path = Path::new(path_str);
                let contents = fs::read(path)?;
                let blob_id = repo.write_blob(&contents)?;

                let mode = if fs::metadata(path)?.permissions().mode() & 0o111 != 0 {
                    EntryKind::BlobExecutable.into()
                } else {
                    EntryKind::Blob.into()
                };

                entries.push(objs::tree::Entry {
                    mode,
                    filename: path
                        .file_name()
                        .ok_or_else(|| anyhow!("Invalid file name"))?
                        .as_encoded_bytes()
                        .to_vec()
                        .into(),
                    oid: blob_id.detach(),
                });
            }

            entries.sort_by(|a, b| a.filename.cmp(&b.filename));
            let tree = objs::Tree { entries };
            let tree_id = repo.write_object(&tree)?;

            let msg = message
                .replace("{current_version}", &current_version)
                .replace("{new_version}", &new_version);

            let mut head_ref = repo
                .head_ref()?
                .ok_or_else(|| anyhow!("No HEAD reference"))?;
            let head_commit = head_ref.peel_to_commit()?;

            let signature = gix::actor::Signature {
                name: head_commit.committer().unwrap().name.into(),
                email: head_commit.committer().unwrap().email.into(),
                time: gix::date::Time::now_utc(),
            };
            let commit = objs::Commit {
                tree: tree_id.detach(),
                parents: SmallVec::from_vec(vec![head_commit.id]),
                author: signature.clone(),
                committer: signature.clone(),
                encoding: None,
                message: msg.into(),
                extra_headers: Vec::new(),
            };

            let commit_id = repo.write_object(&commit)?;

            repo.reference(
                head_ref.name().to_owned(),
                commit_id,
                gix::refs::transaction::PreviousValue::MustExistAndMatch(head_ref.inner.target),
                "Version bump commit",
            )?;

            println!("Committed: {commit_id}");

            if tag {
                let tag_name = format!("v{}", new_version);
                repo.tag_reference(
                    &tag_name,
                    commit_id,
                    gix::refs::transaction::PreviousValue::Any,
                )?;

                println!("Git lightweight tag created: refs/tags/{}", tag_name);
            }
        }
    } else {
        eprintln!("No version bump attempted, and no files specified");
    }

    Ok(())
}
