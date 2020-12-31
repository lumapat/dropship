use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::vec::Vec;

#[derive(Debug)]
pub struct DirTree {
    /// Full path of the current directory
    path: PathBuf,

    /// Files in the current directory
    files: Vec<PathBuf>,

    /// Subdirectories in the current directory
    subdirs: Vec<Box<DirTree>>,
}

pub fn load_dir_tree(path: &Path) -> Result<DirTree, Box<dyn Error>> {
    let mut dir_tree = DirTree {
        path: path.to_path_buf(),
        files: Vec::new(),
        subdirs: Vec::new(),
    };

    for entry in fs::read_dir(path)? {
        let entry_path = entry?.path();
        if entry_path.is_dir() {
            let subdir_tree = load_dir_tree(entry_path.as_path())?;
            dir_tree.subdirs.push(Box::new(subdir_tree));
        } else if entry_path.is_file() {
            dir_tree.files.push(entry_path);
        } else {
            // TODO here
            println!("Unrecognized entry type. Skipping!");
        }
    }

    Ok(dir_tree)
}