pub mod file_hash {
    use std::path::Path;
    use sha256::digest_file;

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
}