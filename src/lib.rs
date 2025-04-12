pub mod file_hash {
    use std::path::Path;
    use sha256::digest_file;
    use walkdir::WalkDir;
    use std::io::Error;

    #[derive(Clone)]
    pub struct FileHash {
        pub path: String,
        pub hash: String,
    }

    #[allow(dead_code)]
    impl<'b> FileHash {
        pub fn new()-> FileHash {
            FileHash { 
                path: String::from("."), 
                hash: String::default()
            }
        }
        
        pub fn from_path(path: &Path) -> Result<FileHash, &str> {
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
    
    pub fn traverse_dir(path: &Path) -> Result<Vec<FileHash>, Error> {
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
    
    pub fn new()-> FileHash {
        FileHash { 
            path: String::from("."), 
            hash: String::default()
        }
    }
    
    pub fn from_path(path: &Path) -> Result<FileHash, &str> {
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

