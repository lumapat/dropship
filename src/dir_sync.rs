use crate::dir_diff;

use log::{debug, info, warn};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::vec::Vec;

// TODO: Entertain the idea of a revertible commit

// TODO: Move PathBufs instead of creating new PathBufs
#[derive(Debug, Clone)]
pub enum SyncOp {
    /// Copy a file or directory from a source location to a destination
    Copy {
        /// Thing to copy
        src: PathBuf,

        /// Destination to copy to
        dest: PathBuf,
    },

    /// Remove a file or directory
    Remove {
        /// Path of file or directory to remove
        path: PathBuf,
    },

    /// Keep the existing file or directory (for verbose/debugging purposes)
    Keep {
        /// Path of file or directory to keep
        path: PathBuf,
    },

    // TODO: Support other operations
    // Move - move a file from src to dest (aka rename)
    // Sync - like copy (for summary/debugging when you want to condense output)
    //      - since copy ignores what's already there, sync means that whatever
    //      - isn't will be copied and whatever is will stay
}

impl fmt::Display for SyncOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyncOp::Copy{src, dest} => write!(f, "Copying '{}' to '{}'", src.display(), dest.display()),
            SyncOp::Keep{path} => write!(f, "Keeping '{}'", path.display()),
            SyncOp::Remove{path} => write!(f, "Removing '{}'", path.display()),
        }
    }
}

// TODO: Define "new" and "missing" as coming from "dir_diff"

#[derive(Debug)]
pub enum SyncStrategy {
    /// Remove new items and add missing items
    Full,
    /// Patch up any missing items but do not delete new items
    Patch,
}

pub fn generate_sync_operations(comparison: &dir_diff::DirComparison,
                                strategy: &SyncStrategy,
                                base_dir: &Path, target_dir: &Path) -> Vec<SyncOp> {
    let mut ops: Vec<SyncOp> = Vec::new();

    // TODO: Understand difference between clone and copy that allowed the assignment
    // under here
    // TODO: Refactor to be a lot more dense
    let file_comparison = &comparison.file_comparison;

    let to_copy_op = |f: String| -> SyncOp {
        let mut base = base_dir.to_owned();
        let mut target = target_dir.to_owned();

        base.push(f.clone());
        target.push(f.clone());

        SyncOp::Copy{
            src: base,
            dest: target,
        }
    };

    let to_remove_op = |f: String| -> SyncOp {
        let mut target = target_dir.to_owned();
        target.push(f.clone());

        SyncOp::Remove{path: target}
    };

    let to_keep_op = |f: String| -> SyncOp {
        let mut target = target_dir.to_owned();
        target.push(f.clone());

        SyncOp::Keep{path: target}
    };

    // TODO: Make sync operations contingent on the type of sync

    // Create operations for files
    ops.extend(file_comparison.missing.clone().into_iter().map(to_copy_op));
    ops.extend(file_comparison.changed.clone().into_iter().map(to_copy_op));
    match strategy {
        SyncStrategy::Full => ops.extend(file_comparison.new.clone().into_iter().map(to_remove_op)),
        _ => ops.extend(file_comparison.new.clone().into_iter().map(to_keep_op)),
    }
    ops.extend(file_comparison.same.clone().into_iter().map(to_keep_op));

    let subdir_comparison = &comparison.subdir_comparison;
    ops.extend(subdir_comparison.missing.clone().into_iter().map(to_copy_op));
    match strategy {
        SyncStrategy::Full => ops.extend(subdir_comparison.new.clone().into_iter().map(to_remove_op)),
        _ => ops.extend(subdir_comparison.new.clone().into_iter().map(to_keep_op)),
    }
    ops.extend(subdir_comparison.same.clone().into_iter().map(to_keep_op));

    let changed_subdirs = &comparison.changed_subdirs;
    for (d, cmp) in changed_subdirs.iter() {
        let mut base = base_dir.to_owned();
        let mut target = target_dir.to_owned();

        base.push(d.clone());
        target.push(d.clone());

        ops.append(&mut generate_sync_operations(&cmp, strategy, &base, &target));
    }

    ops
}

fn copy_item(from: &PathBuf, to: &PathBuf) -> std::io::Result<()> {
    if from.is_file() {
        if to.exists() && !to.is_file() {
            // TODO: Do something here
        }

        // TODO: Check return value is equal to size of file (in bytes)
        fs::copy(from, to)?;
    } else if from.is_dir() {
        if to.exists() {
            // TODO: Do something here
        }

        fs::create_dir_all(to)?;

        for entry in fs::read_dir(from)? {
            let from_subdir_path = entry?.path();

            // TODO: Properly handle these issues
            match from_subdir_path.file_name() {
                Some(os_s) => match os_s.to_str() {
                    Some(file_name) => {
                        let mut to_subdir_path: PathBuf = to.clone();
                        to_subdir_path.push(file_name);

                        copy_item(&from_subdir_path, &to_subdir_path)?;
                    },
                    None => warn!("TODO: Can't read that file's name"),
                },
                None => warn!("TODO: Path ends with '..'"),
            };
        }
    } else {
        debug!("Unsupported operation!");
    }

    Ok(())
}

fn remove_item(path: &PathBuf) -> std::io::Result<()> {
    if path.is_file() {
        fs::remove_file(path)?;
    } else if path.is_dir() {
        for entry in fs::read_dir(path)? {
            remove_item(&entry?.path())?;
        }

        fs::remove_dir(path)?;
    } else {
        warn!("Unsupported operation!");
    }

    Ok(())
}

pub fn commit_sync(ops: &Vec<SyncOp>) -> std::io::Result<()> {
    for op in ops.iter() {
        info!("{}", op);
        match op {
            SyncOp::Copy{src, dest} => copy_item(&src, &dest)?,
            SyncOp::Keep{path: _} => (),
            SyncOp::Remove{path} => remove_item(&path)?,
        }
    }

    Ok(())
}