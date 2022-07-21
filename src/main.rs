#![allow(unused_imports)]

use std::collections::HashMap;
use std::{env, io, string};
use std::io::Error;
use std::fs::{self, File, DirEntry};
use std::path::{Path, PathBuf};
use sha256::digest_file;

fn main() -> io::Result<()> {
    println!("File Duplication Detector");

    // Get args crom command line
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let mut file_hashes: Vec<FileHash> = Vec::new();
    let mut file_hash: &FileHash = &FileHash::new();

    // Check if args supplied
    if args.len() > 1 {
        // Check if first arg is a directory or file
        if path.is_file() {
            println!("Please supply a directory path as first argument");    
        }

        // Run comparison
        println!("Comparing files in {:?}", path.display());
        let mut entry: Result<DirEntry, Error>;
        let mut this_path: PathBuf;
        let mut temp: &FileHash = &FileHash::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let this_path = entry.path();
            if this_path.is_file() {
                file_hash = &FileHash::from_path(this_path.as_path()).unwrap();
                file_hashes.push(FileHash::from_path(this_path.as_path()).unwrap());
            }
        }

        for hash in file_hashes {
           println!("Path: {:?} \t Hash: {:?}", hash.path, hash.hash);
        }

    } else {
        println!("Please supply a directory path as first argument");
    }

    Ok(())
}

// fn read_files(path: &Path) -> &Path {
//     let mut entry: Result<DirEntry, Error>;
//         let mut this_path: PathBuf;
//         let mut temp: &FileHash = &FileHash::new();

//         for entry in fs::read_dir(path)? {
//             let entry = entry?;
//             let this_path = entry.path();
//             if this_path.is_file() {
//                 file_hash = &FileHash::from_path(this_path.as_path()).unwrap();
//                 file_hashes.push(FileHash::from_path(this_path.as_path()).unwrap());
//             } else {
//                 read_files(path);
//             }
//         }
// }

#[derive(Debug)]
struct FileHash {
    path: String,
    hash: String,
}

impl<'b> FileHash {
    fn new()-> FileHash {
        let new = FileHash { path: String::from("."), hash: String::from("") };
        new
        
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