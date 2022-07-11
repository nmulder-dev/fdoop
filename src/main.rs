#![allow(unused_imports)]

use std::{env,fs::File,path::Path};

fn main() {
    println!("File Duplication Detector");

    // Get args crom command line
    let args: Vec<String> = env::args().collect();

    // Check if args supplied
    if args.len() > 1 {
        let path = Path::new(&args[1]);

        // Check if first arg is a directory or file
        if path.is_file() {
            println!("Please supply a directory path as first argument");    
        }

        // Run comparison
        println!("Comparing file in {:?}", path.display());
        

    } else {
        println!("Please supply a directory path as first argument");
    }
}
