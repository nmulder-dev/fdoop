use std::{env, io};
use std::path::Path;
use std::collections::HashSet;
use fdoop::file_hash::*;

fn main() -> io::Result<()> {
    println!("File Duplication Detector");

    // Get args from command line
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);

    // Check if args supplied
    if args.len() > 1 {
        // Check if first arg is a directory or file
        if path.is_file() {
            println!("Please supply a directory path as first argument");
            return Ok(());
        }

        // Run comparison
        println!("Comparing files in {:?}", path.display());

        let mut counter: i32 = 0;

        // Traverse dir and get file hashes
        let file_hashes: Vec<FileHash> = traverse_dir(path).unwrap();
        let mut seen_hashes: HashSet<String> = HashSet::new();

        // Compare each hash with every other hash
        for (i, first_h) in file_hashes.iter().enumerate() {
            if seen_hashes.contains(&first_h.hash) {
                continue;
            }
            for (j, second_h) in file_hashes.iter().enumerate() {
                if i != j && first_h.hash == second_h.hash {
                    counter += 1;
                    seen_hashes.insert(first_h.hash.clone());
                    println!("Duplicate: {:?}", second_h.path);
                    println!("         : {:?}\n", first_h.path);
                    break;
                }
            }
        }
        println!("{:?} duplicates found", counter);

    } else {
        println!("Please supply a directory path as first argument");
    }

    Ok(())
}