use std::error::Error;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

mod dir_diff;
mod dir_sync;
mod dir_tree;

#[derive(Debug, StructOpt)]
#[structopt(name = "options", about = "TODO!")]
struct Opt {
    /// Run without committing any changes
    #[structopt(short, long)]
    no_commit: bool,

    /// Set verbose level
    #[structopt(short, long)]
    verbose: bool,

    /// Run command
    #[structopt(subcommand)]
    command: Command
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Command TODO!")]
enum Command {
    /// Compare contents of pathectories
    Compare {
        /// Directory to base differences from
        #[structopt(short, long)]
        base_path: PathBuf,
        /// Directory to compare with
        #[structopt(short, long)]
        target_path: PathBuf,
    },

    /// Sync contents of pathectories
    #[structopt(name = "sync")]
    Synchronize {
        /// Directory to base differences from
        #[structopt(short, long)]
        base_path: PathBuf,
        /// Directory to compare with
        #[structopt(short, long)]
        target_path: PathBuf,
    }
}

fn compare(base_path: &Path, target_path: &Path) -> Result<(), Box<dyn Error>> {
    let base_dir_tree = dir_tree::load_dir_tree(base_path)?;
    let target_dir_tree = dir_tree::load_dir_tree(target_path)?;

    let comparison = dir_diff::compare_dirs(&base_dir_tree, &target_dir_tree);
    println!("Comparing {:#?} to {:#?}: {:#?}",
             base_path,
             target_path,
             comparison);

    Ok(())
}

fn synchronize(base_path: &Path, target_path: &Path) -> Result<(), Box<dyn Error>> {
    let base_dir_tree = dir_tree::load_dir_tree(base_path)?;
    let target_dir_tree = dir_tree::load_dir_tree(target_path)?;

    println!("Synchronizing {:#?} to {:#?}...",
             target_path,
             base_path);

    let comparison = dir_diff::compare_dirs(&base_dir_tree, &target_dir_tree);
    let sync_ops = dir_sync::generate_sync_operations(&comparison, &base_path, &target_path);

    for op in sync_ops.iter() {
        match op {
            dir_sync::SyncOp::Copy{src, dest} => println!("Copying {:?} to {:?}", src, dest),
            dir_sync::SyncOp::Remove{path} => println!("Removing {:?}", path),
            dir_sync::SyncOp::Keep{path} => println!("Keeping {:?}", path),
        }
    }

    Ok(())
}


fn process_command(cmd: &Command) -> Result<(), Box<dyn Error>> {
    match cmd {
        Command::Compare { base_path, target_path } => compare(base_path.as_path(), target_path.as_path())?,
        Command::Synchronize { base_path, target_path } => synchronize(base_path.as_path(), target_path.as_path())?,
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    process_command(&opt.command)?;

    Ok(())
}
