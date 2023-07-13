// for cli-args
use seahorse::{App, Context, Flag, FlagType};
use std::env;
use std::path::Path;
use std::process::exit;

use itertools::Itertools;

// threading
use rayon::{scope, ThreadPoolBuilder};

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
        );

    app.run(args);
}

fn action(c: &Context) {
    if c.args.is_empty() {
        c.help();
        exit(0);
    }

    // args

    let mut files: Vec<&str> = vec![];
    c.args.iter().for_each(|arg| files.push(arg));
    // remove duplicate files from files vector(this speeds up file-rs a lot)
    let files = files.iter().unique().collect::<Vec<_>>();

    let show_mime_type: bool = c.bool_flag("mime-type");
    let show_extension: bool = c.bool_flag("extension");

    ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build_global()
        .unwrap();

    // main thing
    scope(|s| {
        for file in files.iter() {
            s.spawn(move |_| {
                let path = Path::new(file);

                if !path.exists() {
                    println!("{file:<15}: cannot open '{file}' (No such file, directory or flag)");
                } else {
                    let mut shebang: String = String::new();
                    if !show_mime_type | !show_extension {
                        shebang = inside_file::get_type_from_shebang(path);
                    }
                    // print mime type
                    if show_mime_type {
                        println!("{file:<15}: {:<15}", file_system::get_mime_type(path));
                    }
                    // default output(prints extension)
                    else if show_extension {
                        println!("{file:<15}: {:<15}", file_system::get_file_extension(path));
                    }
                    // if file does not have a shebang
                    else if shebang.is_empty() {
                        println!("{file:<15}: {:<15}", file_system::get_file_type(path));
                    }
                    // if file has a shebang
                    else {
                        println!("{file:<15}: {shebang} script, {}", file_system::get_file_type(path));
                    }
                }
            });
        }
    })
}
