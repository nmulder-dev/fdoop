use std::{env, io};
use std::io::Error;
use std::path::Path;
use sha256::digest_file;
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    println!("File Duplication Detector");

    // Get args f`rom command line
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);

    // Check if args supplied
    if args.len() > 1 {
        // Check if first arg is a directory or file
        if path.is_file() {
            println!("Please supply a directory path as first argument");    
        }

        // Run comparison
        println!("Comparing files in {:?}", path.display());
        
        let mut counter: i32 = 0;

        // traverse dir and get file hashes
        let file_hashes: Vec<FileHash> = traverse_dir(path).unwrap();

        // compare each hash with every other hash
        for (i, hash) in file_hashes.iter().enumerate() {
           for (j, h) in file_hashes.iter().enumerate() {
            if (hash.hash == h.hash) && (i != j) {
                counter += 1;
                println!("Duplicate: {:?}", h.path);
                println!("         : {:?}\n", hash.path)
            }
           }
        }
        println!("{:?} duplicates found", counter);

    } else {
        println!("Please supply a directory path as first argument");
    }

    Ok(())
}

fn traverse_dir(path: &Path) -> Result<Vec<FileHash>, Error> {
    let mut hashes: Vec<FileHash> = Vec::new();
    
    for entry in WalkDir::new(path) {
        let entry_path = entry
                                .as_ref()
                                .unwrap()
                                .path();
        let is_file = entry_path
                            .is_file();
        
        // entry is a file
        if is_file {
            //println!("Hashing file {:?}...", entry_path);
            hashes.push(FileHash::from_path(entry_path).unwrap());
        } 
    }
    
    Ok(hashes)
}

struct FileHash {
    path: String,
    hash: String,
}

#[allow(dead_code)]
impl<'b> FileHash {
    fn new()-> FileHash {
        FileHash { 
            path: String::from("."), 
            hash: String::default()
        }
    }

    fn from_path(path: &Path) -> Result<FileHash, &str> {
        let path = path;
        if path.is_file() {
            let hash = digest_file(path).unwrap();
            let fh: FileHash = FileHash { 
            path: String::from(
                path.to_str().unwrap_or_default()), 
                hash: hash 
            };
            Ok(fh)
        } else {
            Err("Supplied path is not a file")
        }
    }
}