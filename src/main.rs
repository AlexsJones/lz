use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Local};
use clap::Parser;
use tokio::io;
use std::cmp::Reverse;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = ".")]
    path: String,
    /// Include hidden folders (those starting with a dot)
    #[clap(long)]
    hidden: bool,
}

struct FileAccessInfo {
    path: String,
    accessed: SystemTime,
}

fn should_skip_dir(path: &Path, include_hidden: bool) -> bool {
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        if !include_hidden && name.starts_with('.') {
            return true;
        }
        matches!(name, ".cache" | ".cargo" | ".npm" | ".rustup" | ".local" | ".config")
    } else {
        false
    }
}

fn collect_files_parallel(
    dir: &Path,
    results: &mut Vec<FileAccessInfo>,
    pb: &ProgressBar,
    files_scanned: &Arc<AtomicU64>,
    include_hidden: bool
) {
    if should_skip_dir(dir, include_hidden) {
        return;
    }
    // Show the current directory being scanned
    if let Some(dir_name) = dir.file_name().and_then(|n| n.to_str()) {
        pb.set_message(format!("Files counted: {} (scanning: {})", files_scanned.load(Ordering::Relaxed), dir_name));
    } else {
        pb.set_message(format!("Files counted: {} (scanning: {:?})", files_scanned.load(Ordering::Relaxed), dir));
    }
    if let Ok(entries) = fs::read_dir(dir) {
        let entries: Vec<_> = entries.flatten().collect();
        let (files, dirs): (Vec<_>, Vec<_>) = entries.into_iter().partition(|entry| entry.path().is_file());

        // Process files in parallel, batch progress bar updates
        let file_infos: Vec<FileAccessInfo> = files
            .par_iter()
            .map(|entry| {
                let accessed = entry
                    .metadata()
                    .and_then(|m| m.accessed())
                    .unwrap_or(UNIX_EPOCH);
                let count = files_scanned.fetch_add(1, Ordering::Relaxed) + 1;
                if count % 10 == 0 {
                    pb.set_message(format!("Files counted: {}", count));
                }
                FileAccessInfo {
                    path: entry.path().to_string_lossy().into_owned(),
                    accessed,
                }
            })
            .collect();
        results.extend(file_infos);

        // Process subdirectories in parallel
        let sub_results: Vec<Vec<FileAccessInfo>> = dirs
            .par_iter()
            .map(|entry| {
                let mut sub_vec = Vec::new();
                collect_files_parallel(&entry.path(), &mut sub_vec, pb, files_scanned, include_hidden);
                sub_vec
            })
            .collect();
        for mut sub_vec in sub_results {
            results.append(&mut sub_vec);
        }
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let root = Path::new(&args.path);

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(100));

    let files_scanned = Arc::new(AtomicU64::new(0));
    let mut collected = Vec::new();

    pb.set_message("Files counted: 0");
    collect_files_parallel(root, &mut collected, &pb, &files_scanned, args.hidden);
    pb.set_message(format!("Files counted: {}", files_scanned.load(Ordering::Relaxed)));
    pb.finish_with_message(format!("Scan complete. Total files counted: {}", files_scanned.load(Ordering::Relaxed)));

    collected.sort_by_key(|info| Reverse(info.accessed));

    for info in collected.iter().take(5) {
        let dt: DateTime<Local> = info.accessed.into();
        println!("{} ({})", info.path, dt.format("%Y-%m-%d %H:%M:%S"));
    }

    Ok(())
}
