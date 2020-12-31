use std::error::Error;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

mod dir_tree;

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

fn compare(base_path: &Path, target_path: &Path) -> Result<(), Box<dyn Error>> {
    let base_dir_tree = dir_tree::load_dir_tree(base_path)?;
    let target_dir_tree = dir_tree::load_dir_tree(target_path)?;

    println!("Comparing {:#?} to {:#?}",
             base_dir_tree,
             target_dir_tree);

    Ok(())
}

fn synchronize(base_path: &Path, target_path: &Path) -> Result<(), Box<dyn Error>> {
    let base_dir_tree = dir_tree::load_dir_tree(base_path)?;
    let target_dir_tree = dir_tree::load_dir_tree(target_path)?;

    println!("Synchronizing {:#?} to {:#?}",
             base_dir_tree,
             target_dir_tree);

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
