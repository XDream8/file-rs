#![forbid(unsafe_code)]

// for cli-args
use seahorse::{App, Context, Flag, FlagType};
use std::{env, path::Path, process::exit};

use itertools::Itertools;

// threading
use rayon::{prelude::*, ThreadPoolBuilder};

pub mod file_system;
pub mod inside_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage(format!("{} [args] [file(s)]", env!("CARGO_PKG_NAME")))
        .action(action)
        .flag(
            Flag::new("brief", FlagType::Bool)
                .description("do not prepend filenames to output lines")
                .alias("b"),
        )
        .flag(
            Flag::new("extension", FlagType::Bool)
                .description("show file's extension")
                .alias("ext"),
        )
        .flag(
            Flag::new("jobs", FlagType::Int)
                .description("number of jobs to run")
                .alias("j"),
        )
        .flag(
            Flag::new("mime-type", FlagType::Bool)
                .description("show file's mime type")
                .alias("mt"),
        )
        .flag(
            Flag::new("seperator", FlagType::String)
                .description("use string as separator instead of `:'")
                .alias("F"),
        );

    app.run(args);
}

fn action(c: &Context) {
    // show help if no args are passed
    if c.args.is_empty() {
        c.help();
        exit(0);
    }

    // build thread pool according to jobs flag - otherwise rayon decides on how many jobs to use
    if let Ok(jobs) = c.int_flag("jobs") {
        if let Err(err) = ThreadPoolBuilder::new()
            .num_threads(jobs as usize)
            .build_global()
        {
            eprintln!("Failed to build thread pool: {}", err);
            exit(1);
        }
    }

    // collect files and remove duplicates - unique() does not support rayon’s par_iter() method
    let files: Vec<&String> = c.args.iter().unique().collect::<Vec<_>>();

    // other args
    let brief: bool = c.bool_flag("brief");
    let show_mime_type: bool = c.bool_flag("mime-type");
    let show_extension: bool = c.bool_flag("extension");

    let seperator: String = c.string_flag("seperator").unwrap_or(String::from(":"));

    // avoid repeated computations(mini optimization)
    let logging_function = if brief {
        brief_logging
    } else {
        standard_logging
    };

    let file_open_error_function = if brief {
        brief_file_open_error
    } else {
        standard_file_open_error
    };

    // main thing
    files.par_iter().for_each(|file| {
        let path: &Path = Path::new(file);

        if !path.exists() {
            file_open_error_function(file, &seperator)
        } else {
            let info: String = if show_mime_type {
                file_system::get_mime_type(path)
            } else if show_extension {
                file_system::get_file_extension(path)
            } else if let Some(shebang) = inside_file::get_type_from_shebang(path) {
                format!("{shebang} script, {}", file_system::get_file_type(path))
            } else {
                // if file does not have shebang or we encountered an error
                file_system::get_file_type(path)
            };

            // Print information
            logging_function(file, &seperator, &info)
        }
    });
}

fn standard_logging(file: &String, seperator: &String, info: &String) {
    println!("{file:<15}{seperator} {info:<15}")
}
fn brief_logging(_: &String, _: &String, info: &String) {
    println!("{info}")
}

fn standard_file_open_error(file: &String, seperator: &String) {
    eprintln!("{file:<15}{seperator} cannot open '{file}' (No such file, directory or flag)");
}

fn brief_file_open_error(file: &String, _: &String) {
    eprintln!("cannot open '{file}' (No such file, directory or flag)");
}
