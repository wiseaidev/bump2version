// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Watch Mode
//!
//! Provides [`run_watch`], a blocking loop that monitors a set of paths for
//! file-system changes and triggers a version bump on every detected
//! modification.
//!
//! This module requires the `watch` Cargo feature.
//!
//! ## Example
//!
//! ```no_run
//! # #[cfg(feature = "watch")]
//! use bump2version::watch::run_watch;
//! use bump2version::version::BumpPart;
//! # #[cfg(feature = "watch")]
//! run_watch(".bumpversion.toml", &BumpPart::Patch).unwrap();
//! ```

use crate::config::parse_config_file;
use crate::error::BumpError;
use crate::files::{apply_config_version_update, apply_file_change};
use crate::utils::{collect_file_configs, compute_new_version};
use crate::version::BumpPart;
use notify::{Event, EventKind, RecursiveMode, Watcher, recommended_watcher};
use std::fs;
use std::path::Path;
use std::sync::mpsc;
use std::time::{Duration, Instant};

/// Minimum quiet time between two successive bumps.
const DEBOUNCE_MS: u64 = 500;

/// Starts a blocking file-system watch loop.
///
/// Watches `config_path` and all files listed in its `[bumpversion:file:...]`
/// sections for modifications.  On every write event, the version is bumped
/// by `part` (e.g. `"patch"`) and all registered files are updated.
///
/// Press Ctrl-C to stop.
///
/// # Arguments
///
/// * `config_path` - Path to the `.bumpversion.toml` configuration file.
/// * `part` - Version component to bump on each save.
///
/// # Errors
///
/// Returns [`BumpError::Other`] when the file-system watcher cannot be
/// created, or propagates errors from the bump pipeline.
///
/// # Complexity
///
/// - **Time**: O(∞) - blocks until interrupted.
/// - **Space**: O(F) where F = number of watched files.
pub fn run_watch(config_path: &str, part: &BumpPart) -> Result<(), BumpError> {
    let (tx, rx) = mpsc::channel::<Result<Event, notify::Error>>();

    let mut watcher = recommended_watcher(move |res| {
        let _ = tx.send(res);
    })
    .map_err(|e| BumpError::Other(format!("watcher init: {e}")))?;

    watcher
        .watch(Path::new(config_path), RecursiveMode::NonRecursive)
        .map_err(|e| BumpError::Other(format!("watch config: {e}")))?;

    let initial_cfg = parse_config_file(config_path)?;
    for fc in &initial_cfg.files {
        if Path::new(&fc.path).exists() {
            watcher
                .watch(Path::new(&fc.path), RecursiveMode::NonRecursive)
                .map_err(|e| BumpError::Other(format!("watch {}: {e}", fc.path)))?;
        }
    }

    println!("[watch] Watching for changes. Press Ctrl-C to stop.");

    let mut last_bump: Option<Instant> = None;

    for event in &rx {
        let event = event.map_err(|e| BumpError::Other(e.to_string()))?;

        if !matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
            continue;
        }

        if last_bump
            .map(|t| t.elapsed() < Duration::from_millis(DEBOUNCE_MS))
            .unwrap_or(false)
        {
            continue;
        }

        let cfg = match parse_config_file(config_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[watch] Config error: {e}");
                continue;
            }
        };

        let current = match &cfg.current_version {
            Some(v) => v.clone(),
            None => {
                eprintln!("[watch] current_version not set in config.");
                continue;
            }
        };

        let new_version = match compute_new_version(config_path, part, None, None, None) {
            Some(v) => v,
            None => {
                eprintln!("[watch] Could not compute new version.");
                continue;
            }
        };

        let file_configs = collect_file_configs(&cfg, &[]);
        let mut any_updated = false;

        for fc in &file_configs {
            match fs::read_to_string(&fc.path) {
                Ok(content) => {
                    match apply_file_change(&content, fc, &cfg, &current, &new_version) {
                        Ok(updated) => {
                            if let Err(e) = fs::write(&fc.path, &updated) {
                                eprintln!("[watch] Write error {}: {e}", fc.path);
                            } else {
                                any_updated = true;
                            }
                        }
                        Err(e) => eprintln!("[watch] Replace error {}: {e}", fc.path),
                    }
                }
                Err(e) => eprintln!("[watch] Read error {}: {e}", fc.path),
            }
        }

        if any_updated {
            let config_content = match fs::read_to_string(config_path) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let updated_config =
                apply_config_version_update(&config_content, &current, &new_version);
            let _ = fs::write(config_path, &updated_config);

            last_bump = Some(Instant::now());

            while rx.try_recv().is_ok() {}

            println!("[watch] Bumped {current} → {new_version}");
        }
    }

    Ok(())
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
