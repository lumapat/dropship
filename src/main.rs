use log::info;
use std::error::Error;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

mod dir_diff;
mod dir_sync;
mod dir_tree;

#[derive(Debug, StructOpt)]
#[structopt(name = "global options", about = "TODO!")]
struct GlobalOptions {
    /// Run without committing any changes
    #[structopt(short, long)]
    no_commit: bool,

    /// Set verbose level
    #[structopt(short, long)]
    verbose: bool,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "arguments", about = "TODO!")]
struct Args {
    #[structopt(flatten)]
    globals: GlobalOptions,

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
    info!("Comparing {:#?} to {:#?}: {:#?}",
             base_path,
             target_path,
             comparison);

    Ok(())
}

fn synchronize(base_path: &Path, target_path: &Path, globals: &GlobalOptions) -> Result<(), Box<dyn Error>> {
    let base_dir_tree = dir_tree::load_dir_tree(base_path)?;
    let target_dir_tree = dir_tree::load_dir_tree(target_path)?;

    info!("Synchronizing {:#?} to {:#?}...",
             target_path,
             base_path);

    let comparison = dir_diff::compare_dirs(&base_dir_tree, &target_dir_tree);
    let sync_ops = dir_sync::generate_sync_operations(&comparison, &base_path, &target_path);

    if globals.no_commit {
        info!("No-commit enabled. Not committing!");
        for op in sync_ops.iter() {
            info!("{}", op);
        }
    } else {
        info!("Committing...");
        dir_sync::commit_sync(&sync_ops)?;
    }

    Ok(())
}


fn process_command(cmd: &Command, globals: &GlobalOptions) -> Result<(), Box<dyn Error>> {
    match cmd {
        Command::Compare { base_path, target_path } => compare(base_path.as_path(), target_path.as_path())?,
        Command::Synchronize { base_path, target_path } => synchronize(base_path.as_path(), target_path.as_path(), globals)?,
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::SimpleLogger::new().init().unwrap();

    let args = Args::from_args();
    process_command(&args.command, &args.globals)?;

    Ok(())
}
