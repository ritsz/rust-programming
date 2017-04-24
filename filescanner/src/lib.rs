/*
   The idea is that this library will be called from a C/C++ function.
   This would mean, given a String path, scan it completely and write metadata to file.

   1. This should be able to handle multiple threads call this. 
   2. Given a directory path, scan all files and add dirs to the list.
 */

use std::fs::{self, DirEntry};
use std::path::Path;
use std::os::unix::fs::MetadataExt;

pub extern "C" fn scan(dir: &str) {
    let path = Path::new(dir);
    println!("Checking path {:?}", path.to_str().unwrap());
    /* This scan function will be called only for dir */
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        match path.is_dir() {
            true => scan(&path.to_str().unwrap()),
            false => handle_files(&entry),
        }
    }
}

fn handle_files(file: &DirEntry) {
    let meta = file.metadata().unwrap();
    println!("File: {:?} Time {:?}", file.path().to_str().unwrap(), meta.mtime());
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        scan(&"/home/ritsz/Programming/Rust/filescanner");
    }
}
