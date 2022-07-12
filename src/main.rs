#![allow(unused_imports)]

use std::collections::HashMap;
use std::{env, io};
use std::fs::{self, File, DirEntry};
use std::path::Path;
use sha256::digest_file;

fn main() -> io::Result<()> {
    println!("File Duplication Detector");

    // Get args crom command line
    let args: Vec<String> = env::args().collect();

    // Check if args supplied
    if args.len() > 1 {
        let path = Path::new(&args[1]);
        let mut file_hashes: Vec<FileHash> = Vec::new();

        // Check if first arg is a directory or file
        if path.is_file() {
            println!("Please supply a directory path as first argument");    
        }

        // Run comparison
        println!("Comparing files in {:?}", path.display());
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let this_path = entry.path();
            if this_path.is_file() {
                let hash = digest_file(this_path).unwrap();
                file_hashes.push(FileHash {path: Box::new(this_path.as_path()), hash: hash});
                println!("{:?}", hash);
            }
        }

    } else {
        println!("Please supply a directory path as first argument");
    }

    Ok(())
}

#[derive(Debug)]
struct FileHash {
    path: Box<&'a Path>,
    hash: String,
}
