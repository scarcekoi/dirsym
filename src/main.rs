use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The source directory that contains the file(s) to link to
    source: PathBuf,
    /// The destination directory where the symlink(s) are created
    destination: PathBuf,
}

fn main() {
    let args = Args::parse();
}

fn find_files(path: PathBuf) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();

    for file in path.read_dir().expect("failed to read path").flatten() {
        files.push(file.path());
    }

    files
}
