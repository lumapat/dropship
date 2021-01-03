use crate::dir_tree::DirTree;
use std::clone::Clone;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug)]
pub struct Comparison<T> {
    /// Items that contain differences
    pub changed: HashSet<T>,

    /// Items that are missing from the target
    pub missing: HashSet<T>,

    /// Items that are new in the target
    pub new: HashSet<T>,

    /// Items that are identical between base and target
    pub same: HashSet<T>,
}

impl<T> Comparison<T> {
    pub fn unchanged(&self) -> bool {
        self.changed.is_empty() &&
        self.new.is_empty() &&
        self.missing.is_empty()
    }
}

#[derive(Debug)]
pub struct DirComparison {
    /// Result of comparing files in the directory
    pub file_comparison: Comparison<String>,

    /// Result of comparing direct subdirectories
    pub subdir_comparison: Comparison<String>,

    /// Recursive comparisons on any changed subdirectories
    pub changed_subdirs: HashMap<String, Box<DirComparison>>,
}

impl DirComparison {
    pub fn unchanged(&self) -> bool {
        self.file_comparison.unchanged() &&
        self.subdir_comparison.unchanged() &&
        self.changed_subdirs.is_empty()
    }
}

fn simple_comparison<T: Clone + Eq + Hash>(base: &HashSet<T>, target: &HashSet<T>) -> Comparison<T> {
    Comparison {
        changed: HashSet::new(),
        missing: base.symmetric_difference(target).cloned().collect(),
        new: target.symmetric_difference(base).cloned().collect(),
        same: base.intersection(target).cloned().collect(),
    }
}

pub fn compare_dirs(base_dir: &DirTree, target_dir: &DirTree) -> DirComparison {
    // Assuming directory names are the same, compare contents

    // Compare files first
    let base_file_names: HashSet<_> = base_dir.files.keys().cloned().collect();
    let target_file_names: HashSet<_> = target_dir.files.keys().cloned().collect();

    let file_comparison = simple_comparison(&base_file_names, &target_file_names);

    // Compare subdirectories
    let base_subdir_names: HashSet<_> = base_dir.subdirs.keys().cloned().collect();
    let target_subdir_names: HashSet<_> = target_dir.subdirs.keys().cloned().collect();

    let subdir_prelim_comparison = simple_comparison(&base_subdir_names, &target_subdir_names);

    // Recursively compare all shared subdirectories
    let mut changed_subdirs: HashMap<String, Box<DirComparison>> = HashMap::new();
    for subdir in subdir_prelim_comparison.same.iter() {
        let base_subdir = match base_dir.subdirs.get(subdir) {
            Some(d) => d,
            None => {
                println!("Somehow you messed up with {:?}", subdir);
                continue;
            }
        };

        let target_subdir = match target_dir.subdirs.get(subdir) {
            Some(d) => d,
            None => {
                println!("Somehow you messed up with {:?}", subdir);
                continue;
            }
        };

        let subdir_comparison = compare_dirs(&base_subdir, &target_subdir);

        if !subdir_comparison.unchanged() {
            changed_subdirs.insert(subdir.to_string(), Box::new(subdir_comparison));
        }
    }

    let mut subdir_comparison = subdir_prelim_comparison;

    subdir_comparison.changed = changed_subdirs.keys().cloned().collect();
    subdir_comparison.same = subdir_comparison.same.symmetric_difference(&subdir_comparison.changed).cloned().collect();

    DirComparison {
        file_comparison,
        subdir_comparison,
        changed_subdirs,
    }
}