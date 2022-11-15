// for cli-args
use seahorse::{App, Context};
use std::{env, process::exit};

// for getting file ext
use std::{ffi::OsStr, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage(format!("{} [file(s)]", env!("CARGO_PKG_NAME")))
        .action(action);

    app.run(args);
}

fn action(c: &Context) {
    // args
    let files = match c.args.len() {
        1 => &c.args[0],
        _ => {
            c.help();
            exit(0);
        }
    };

    // main thing
    for file in files.split_whitespace() {
        let extension: String = match get_file_extension(file) {
            Some(ext) => format!("{}", ext).to_string(),
            None => "[have no extension]".to_string(),
        };
        println!("File: {}, ext: {}", file, extension);
    }
}

fn get_file_extension(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}
