use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Local};
use clap::Parser;
use tokio::io;
use std::cmp::Reverse;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = ".")]
    path: String,
}

struct FileAccessInfo {
    path: String,
    accessed: SystemTime,
}

fn visit_dirs<F>(dir: &Path, cb: &mut F) -> io::Result<()>
where
    F: FnMut(FileAccessInfo),
{
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                // SAFELY get access time now, drop fd immediately
                let accessed = entry
                    .metadata()
                    .and_then(|m| m.accessed())
                    .unwrap_or(UNIX_EPOCH);

                cb(FileAccessInfo {
                    path: path.to_string_lossy().into_owned(),
                    accessed,
                });
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut collected = Vec::new();

    visit_dirs(Path::new(&args.path), &mut |info| {
        collected.push(info);
    })?;

    collected.sort_by_key(|info| Reverse(info.accessed));

    for info in collected.iter().take(5) {
        let dt: DateTime<Local> = info.accessed.into();
        println!("{} ({})", info.path, dt.format("%Y-%m-%d %H:%M:%S"));
    }

    Ok(())
}
