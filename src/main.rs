use std::path::PathBuf;

use clap::Parser;
use symlink::symlink_file;

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

    create_symlinks(find_files(args.source), args.destination);
}

fn find_files(path: PathBuf) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();

    for file in path.read_dir().expect("failed to read path").flatten() {
        if file.path().is_file() {
            files.push(file.path());
        }
    }

    files
}

fn create_symlinks(files: Vec<PathBuf>, destination: PathBuf) {
    for file in files {
        let file_name = file.file_name().expect("failed to get file name");
        let destination: PathBuf = destination.join(file_name);

        symlink_file(&file, destination).expect("failed to create symlink");
    }
}
