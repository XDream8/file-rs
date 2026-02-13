#![forbid(unsafe_code)]

// for cli-args
use argh::FromArgs;
use std::{path::PathBuf, process::exit};

use itertools::Itertools;

// threading
use rayon::{prelude::*, ThreadPoolBuilder};

pub mod file_system;
pub mod inside_file;

#[derive(FromArgs)]
/// a tool for determining file types
struct Cli {
    /// do not prepend filenames to output lines
    #[argh(switch, short = 'b')]
    brief: bool,

    /// show file's extension
    #[argh(switch, short = 'e')]
    extension: bool,

    /// show file's mime type
    #[argh(switch, short = 'm')]
    mime_type: bool,

    /// number of jobs to run
    #[argh(option, short = 'j')]
    jobs: Option<usize>,

    /// use string as separator instead of `:'
    #[argh(option, short = 'F', default = "String::from(\":\")")]
    separator: String,

    /// file's to process
    #[argh(positional, greedy)]
    files: Vec<PathBuf>,
}

fn main() {
    let cli: Cli = argh::from_env();

    // at least one file input is required
    if cli.files.is_empty() {
        eprintln!("Error: at least one file input is required.");
        std::process::exit(1);
    }

    // build thread pool according to jobs flag - otherwise rayon decides on how many jobs to use
    if let Some(jobs) = cli.jobs {
        if let Err(err) = ThreadPoolBuilder::new().num_threads(jobs).build_global() {
            eprintln!("Error: Failed to build thread pool: {}", err);
            exit(1);
        }
    }

    // collect files and remove duplicates - unique() does not support rayon’s par_iter() method
    let files: Vec<&PathBuf> = cli.files.iter().unique().collect::<Vec<_>>();

    // avoid repeated computations(mini optimization)
    let logging_function = if cli.brief {
        brief_logging
    } else {
        standard_logging
    };

    let file_open_error_function = if cli.brief {
        brief_file_open_error
    } else {
        standard_file_open_error
    };

    // main thing
    files.par_iter().for_each(|file| {
        let file_path: String = file.display().to_string();
        if !file.exists() {
            file_open_error_function(&file_path, &cli.separator)
        } else {
            let info: String = if cli.mime_type {
                file_system::get_mime_type(file)
            } else if cli.extension {
                file_system::get_file_extension(file)
            } else if let Some(shebang) = inside_file::get_type_from_shebang(file) {
                format!("{shebang} script, {}", file_system::get_file_type(file))
            } else {
                // if file does not have shebang or we encountered an error
                file_system::get_file_type(file)
            };

            // Print information
            logging_function(&file_path, &cli.separator, &info)
        }
    });
}

fn standard_logging(file: &String, separator: &String, info: &String) {
    println!("{file:<15}{separator} {info:<15}")
}
fn brief_logging(_: &String, _: &String, info: &String) {
    println!("{info}")
}

fn standard_file_open_error(file: &String, separator: &String) {
    eprintln!("{file:<15}{separator} cannot open '{file}' (No such file, directory or flag)");
}

fn brief_file_open_error(file: &String, _: &String) {
    eprintln!("cannot open '{file}' (No such file, directory or flag)");
}
