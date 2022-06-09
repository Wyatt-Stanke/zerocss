use std::collections::HashSet;
use std::path::PathBuf;

use glob::glob;

pub fn search(glob_path: &str) -> HashSet<PathBuf> {
    let mut result = HashSet::new();
    for entry in glob(glob_path).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                result.insert(path);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
    result
}
