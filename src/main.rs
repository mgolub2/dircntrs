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
    let dir_entry = std::fs::read_dir(dir);  
    match dir_entry {
        Err(_) => return 0,
        Ok(dir) => {
            for entry in dir {
                match entry {
                    Err(_) => continue,
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_file() {
                            count += 1;
                        }
                        else if path.is_dir() {
                            count += count_dir(&path);
                            
                        }
                    }
                };
            }
        }
        
    }
    count
}

fn main() {
    let cli = Opts::parse();
    let count = count_dir(&cli.dir);
    println!("{} files in {}", count, cli.dir.display());
}
