use clap::Parser;
use std::fs::{self, DirEntry};
use std::path::Path;
use chrono::{DateTime, Local};
use tokio::io;
use std::cmp::Reverse;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = ".")]
    path: String,
}

fn visit_dirs<F>(dir: &Path, cb: &mut F) -> io::Result<()>
where F: FnMut(DirEntry){
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(entry);
            }
        }
    }
    Ok(())
}
fn collect_entry(vec: &mut Vec<DirEntry>, entry: DirEntry) {
    vec.push(entry);
}
 fn main() -> io::Result<()> {
    let args = Args::parse();
     let mut collected_entries = Vec::new();

     visit_dirs(Path::new(&args.path), &mut |entry| {
         collect_entry(&mut collected_entries, entry);
     })?;
     // Sort these entries by the last access time
     collected_entries.sort_by_key(|entry| Reverse(entry.metadata().unwrap().accessed().unwrap()));
     // print the last 5 accessed
     let formatted = collected_entries.iter().take(5).map(|x| {
         let t =  x.metadata().unwrap().accessed().unwrap();
         let date_time: DateTime<Local> = t.into();
         format!("{}  ({})", x.path().to_str().unwrap().to_owned(), date_time.format("%Y-%m-%d %H:%M:%S"))
     }).collect::<Vec<_>>();
     println!("{}", formatted.join("\n"));
     Ok(())
}

