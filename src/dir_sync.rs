use crate::dir_diff;

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

pub fn generate_sync_operations(comparison: &dir_diff::DirComparison, base_dir: &Path, target_dir: &Path) -> Vec<SyncOp> {
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
    ops.extend(file_comparison.new.clone().into_iter().map(to_remove_op));
    ops.extend(file_comparison.same.clone().into_iter().map(to_keep_op));

    let subdir_comparison = &comparison.subdir_comparison;
    ops.extend(subdir_comparison.missing.clone().into_iter().map(to_copy_op));
    ops.extend(subdir_comparison.new.clone().into_iter().map(to_remove_op));
    ops.extend(subdir_comparison.same.clone().into_iter().map(to_keep_op));

    let changed_subdirs = &comparison.changed_subdirs;
    for (d, cmp) in changed_subdirs.iter() {
        let mut base = base_dir.to_owned();
        let mut target = target_dir.to_owned();

        base.push(d.clone());
        target.push(d.clone());

        ops.append(&mut generate_sync_operations(&cmp, &base, &target));
    }

    ops
}

pub fn _commit_sync(_ops: &Vec<SyncOp>) -> std::io::Result<()> {
    Ok(())
}