// for cli-args
use seahorse::{App, Context, Flag, FlagType};
use std::env;
use std::path::Path;
use std::process::exit;

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
        .usage(format!("{} [file(s)] [args]", env!("CARGO_PKG_NAME")))
        .action(action)
        .flag(
            Flag::new("mime-type", FlagType::Bool)
                .description("show file's mime type")
                .alias("mt"),
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
        );

    app.run(args);
}

fn action(c: &Context) {
    if c.args.is_empty() {
        c.help();
        exit(0);
    }

    // get number of cpus
    let jobs: usize = match c.int_flag("jobs") {
        Ok(jobs) => jobs as usize,
        Err(_) => match std::thread::available_parallelism() {
            Ok(jobs) => usize::from(jobs),
            Err(err) => {
                eprintln!("Failed to detect number of cpus: {}", err);
                exit(1);
            }
        }
    };

    // build thread pool
    if let Err(err) = ThreadPoolBuilder::new().num_threads(jobs).build_global() {
        eprintln!("Failed to build thread pool: {}", err);
        exit(1);
    }

    // collect files and remove duplicates - unique() does not support rayon’s par_iter() method
    let files: Vec<&str> = c.args.par_iter()
        .map(|file| file.as_str())
        .collect::<Vec<_>>()
        .into_iter()
        .unique()
        .collect::<Vec<_>>();

    // other args
    let show_mime_type: bool = c.bool_flag("mime-type");
    let show_extension: bool = c.bool_flag("extension");

    // main thing - preserve print order(mostly) using .enumerate()
    files.par_iter().for_each(|file| {
        let path: &Path = Path::new(file);

        if !path.exists() {
            eprintln!("{file:<15}: cannot open '{file}' (No such file, directory or flag)");
        } else {
            let mut shebang: String = String::new();
            let mut file_type: String = String::new();
            if !show_mime_type | !show_extension {
                shebang = inside_file::get_type_from_shebang(path);
                file_type = file_system::get_file_type(path);
            }

            // Print information
            if show_mime_type {
                println!("{file:<15}: {:<15}", file_system::get_mime_type(path));
            } else if show_extension {
                println!("{file:<15}: {:<15}", file_system::get_file_extension(path));
            } else if shebang.is_empty() {
                println!("{file:<15}: {:<15}", file_type);
            } else {
                println!("{file:<15}: {shebang} script, {}", file_type);
            }
        }
    });
}
