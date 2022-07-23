#![allow(unused_imports)]

use std::collections::HashMap;
use std::{env, io, string};
use std::io::Error;
use std::fs::{self, File, DirEntry};
use std::path::{Path, PathBuf};
use sha256::digest_file;
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    println!("File Duplication Detector");

    // Get args crom command line
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let mut file_hashes: Vec<FileHash> = Vec::new();

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

        file_hashes = traverse_dir(path).unwrap();

        for hash in file_hashes {
           println!("Path: {:?} \t Hash: {:?}", hash.path, hash.hash);
        }

    } else {
        println!("Please supply a directory path as first argument");
    }

    Ok(())
}

fn traverse_dir(path: &Path) -> Result<Vec<FileHash>, Error> {
    let mut entry: Result<DirEntry, Error>;
    let mut this_path: PathBuf = path.to_path_buf();
    let mut temp: &FileHash = &FileHash::new();
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
            println!("Hashing file {:?}...", entry_path);
            hashes.push(FileHash::from_path(entry_path).unwrap());
        } 
    }
    
    Ok(hashes)
}

#[derive(Debug)]
#[derive(Clone)]
struct FileHash {
    path: String,
    hash: String,
}

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