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
    let files = match c.args.len() {
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
            let mime: String = match mime_guess::from_path(file).first() {
                Some(mime) =>  format!("{}", mime).to_string(),
                _ => "???".to_string(),
            };
            println!("{}: {}", file, mime);
        }
        // default output(prints extension)
        else {
            let extension: String = match get_file_extension(file) {
                Some(ext) => format!("{}", ext).to_string(),
                None => "???".to_string(),
            };
            println!("{}: {}", file, extension);
        }
    }
}

fn get_file_extension(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}
