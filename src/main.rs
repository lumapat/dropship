use std::path::{Path, PathBuf};
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
    /// Compare contents of directories
    Compare {
        /// Directory to base differences from
        #[structopt(short, long)]
        base_dir: PathBuf,
        /// Directory to compare with
        #[structopt(short, long)]
        target_dir: PathBuf,
    },

    /// Sync contents of directories
    #[structopt(name = "sync")]
    Synchronize {
        /// Directory to base differences from
        #[structopt(short, long)]
        base_dir: PathBuf,
        /// Directory to compare with
        #[structopt(short, long)]
        target_dir: PathBuf,
    }
}

fn compare(base_dir: &Path, target_dir: &Path) {
    println!("Comparing {} to {}",
             base_dir.display().to_string(),
             target_dir.display().to_string());
}

fn synchronize(base_dir: &Path, target_dir: &Path) {
    println!("Syncing {} to {}",
             base_dir.display().to_string(),
             target_dir.display().to_string());
}

fn process_command(cmd: &Command) {
    match cmd {
        Command::Compare { base_dir, target_dir } => compare(base_dir.as_path(), target_dir.as_path()),
        Command::Synchronize { base_dir, target_dir } => synchronize(base_dir.as_path(), target_dir.as_path()),
    }
}

fn main() {
    let opt = Opt::from_args();

    process_command(&opt.command)
}
