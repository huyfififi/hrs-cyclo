use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::PathBuf;

use clap::Parser;

mod cyclomatic_complexity;
use cyclomatic_complexity::count;
use cyclomatic_complexity::print_result;
mod common;
use common::filter_python_files;
use common::get_all_paths_in_directory;
use common::read_lines;

#[derive(Parser)]
#[command(name = "Cyclon")]
#[command(version = "0.0.1")]
#[command(author = "Kazuki Kijima <kazukiyoshida96@gmail.com>")]
#[command(about = "Check cyclomatic complexity")]
struct Args {
    #[arg(index(1))]
    path: String,
    // TODO: Add optiosn to show only problematic functions
}

fn main() {
    let path: String = Args::parse().path;
    let base_path = PathBuf::from(&path);
    if base_path.is_file() {
        let contents: Vec<String> = read_lines(&base_path);
        let result: HashMap<&str, u8> = count(&contents);
        print_result(
            base_path
                .file_name()
                .unwrap_or(OsStr::new("invalid path"))
                .to_str()
                .unwrap_or("invalid str"),
            &result,
        );
    } else {
        let paths: Vec<PathBuf> = get_all_paths_in_directory(&base_path);
        let python_files: Vec<PathBuf> = filter_python_files(&paths);
        for path in python_files {
            let contents: Vec<String> = read_lines(&path);
            let result: HashMap<&str, u8> = count(&contents);
            print_result(path.file_name().unwrap().to_str().unwrap(), &result);
        }
    }
}
