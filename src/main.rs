// for cli-args
use seahorse::{App, Context, Flag, FlagType};
use std::{env, process::exit};

// for getting file ext
use std::{ffi::OsStr, path::Path};

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
            .alias("mt")
             );

    app.run(args);
}

fn action(c: &Context) {
    // args
    let files: &str = match c.args.len() {
        1 => &c.args[0],
        _ => {
            c.help();
            exit(0);
        }
    };

    let show_mime_type: bool = c.bool_flag("mime-type");

    // main thing
    for file in files.split_whitespace() {
        // print mime type
        if show_mime_type {
            println!("{}: {}", file, get_mime_type(file));
        }
        // default output(prints extension)
        else {
            println!("{}: {}", file, get_file_extension(file));
        }
    }
}

fn get_mime_type(filename: &str) -> String {
    let mime_type: String = match mime_guess::from_path(filename).first() {
        Some(mime) =>  format!("{}", mime).to_string(),
        _ => "???".to_string(),
    };

    return mime_type
}

fn get_file_extension(filename: &str) -> String {
    let extension: String = match Path::new(filename).extension().and_then(OsStr::to_str) {
        Some(ext) => format!("{}", ext).to_string(),
        None => "???".to_string(),
    };

    return extension
}
