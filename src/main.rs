use std::path::PathBuf;

use clap::{Parser, command};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    /// The directory to count files in                                                                   
    dir: PathBuf,                                                                                                                                                                                          
}

fn count_dir(dir: &PathBuf) -> u64 {
    let mut count = 0;
    let mut handles = vec![];    
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            count += 1;
        }
        else if path.is_dir() {
            handles.push(std::thread::spawn(move || {
                count_dir(&path)
            }));
        }
    }
    for handle in handles {
        count += handle.join().unwrap();
    }
    count
}

fn main() {
    let cli = Opts::parse();
    let count = count_dir(&cli.dir);
    println!("{} files in {}", count, cli.dir.display());
}
