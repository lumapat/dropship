use log::warn;

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct File {
    /// Full path of the current directory
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct DirTree {
    /// Full path of the current directory
    pub path: PathBuf,

    /// Files in the current directory
    pub files: HashMap<String, Box<File>>,

    /// Subdirectories in the current directory
    pub subdirs: HashMap<String, Box<DirTree>>,
}

pub fn load_dir_tree(path: &Path) -> Result<DirTree, Box<dyn Error>> {
    let mut dir_tree = DirTree {
        path: path.to_path_buf(),
        files: HashMap::new(),
        subdirs: HashMap::new(),
    };

    for entry in fs::read_dir(path)? {
        let entry_path = entry?.path();

        // TODO: Handle invalid Unicode errors
        let entry_path_str = match entry_path.file_name() {
            Some(oss) => match oss.to_str() {
                Some(s) => String::from(s),
                None => {
                    warn!("Unreadable file name '{:?}'. Skipping!", oss);
                    continue;
                }
            }
            None => {
                warn!("Invalid file or directory '{:?}'. Skipping!", entry_path);
                continue;
            }
        };

        if entry_path.is_dir() {
            let subdir_tree = load_dir_tree(entry_path.as_path())?;
            dir_tree.subdirs.insert(entry_path_str, Box::new(subdir_tree));
        } else if entry_path.is_file() {
            dir_tree.files.insert(entry_path_str, Box::new(File{path: entry_path}));
        } else {
            // TODO here
            warn!("Unrecognized entry type. Skipping!");
        }
    }

    Ok(dir_tree)
}