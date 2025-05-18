use std::ffi::OsString;
use std::fs::{self, ReadDir};
use std::path::Path;

pub struct SearchPath {
    pub files: Vec<OsString>,
    pub folders: Vec<OsString>,
}

// Reads in a given directory path and saves all files & folders in that path as a string
pub fn is_path_valid(path: &Path) -> bool {
    println!("Trying to read: {}", path.display());

    if !path.exists() {
        eprintln!("Path does not exist : {}", path.display());
        return false;
    }
    match fs::read_dir(path) {
        Ok(_) => true,
        Err(e) => {
            eprint!("Failed to read directory: {}: {}", path.display(), e);
            return false;
        }
    }
}

impl SearchPath {
    pub fn new() -> Self {
        SearchPath {
            files: Vec::new(),
            folders: Vec::new(),
        }
    }
    pub fn populate(&mut self, entries: ReadDir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() == true {
                    self.files.push(entry.file_name());
                } else if file_type.is_dir() {
                    self.folders.push(entry.file_name());
                }
            }
        }
        self.files.sort();
        self.folders.sort();
    }

    pub fn print_files_folders(&mut self) {
        println!("Files:");
        for file in self.files.iter() {
            println!("{}", file.to_string_lossy());
        }
        println!("Folders:");
        for folder in self.folders.iter() {
            println!("{}", folder.to_string_lossy());
        }
    }
}
