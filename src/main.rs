use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::vec::Vec;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "options", about = "TODO!")]
struct Opt {
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

#[derive(Debug)]
struct DirTree {
    /// Full path of the current directory
    path: PathBuf,

    /// Files in the current directory
    files: Vec<PathBuf>,

    /// Subdirectories in the current directory
    subdirs: Vec<Box<DirTree>>,
}

fn compare(base_path: &Path, target_path: &Path) -> Result<(), Box<dyn Error>> {
    let base_dir_tree = load_dir_tree(base_path)?;
    let target_dir_tree = load_dir_tree(target_path)?;

    println!("Comparing {:#?} to {:#?}",
             base_dir_tree,
             target_dir_tree);

    Ok(())
}

fn synchronize(base_path: &Path, target_path: &Path) -> Result<(), Box<dyn Error>> {
    let base_dir_tree = load_dir_tree(base_path)?;
    let target_dir_tree = load_dir_tree(target_path)?;

    println!("Synchronizing {:#?} to {:#?}",
             base_dir_tree,
             target_dir_tree);

    Ok(())
}

fn load_dir_tree(path: &Path) -> Result<DirTree, Box<dyn Error>> {
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
