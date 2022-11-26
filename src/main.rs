// for cli-args
use seahorse::{App, Context, Flag, FlagType};
use std::env;

// for getting file ext
use std::{ffi::OsStr, path::Path};

// for getting file type
use std::fs;
use std::os::unix::fs::FileTypeExt;

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
    // args
    let files = &c.args;
    let show_mime_type: bool = c.bool_flag("mime-type");
    let show_extension: bool = c.bool_flag("extension");

    // main thing
    files
        .iter()
        .skip_while(|file| {
            if is_exists(file) == false {
                println!(
                    "{filename:<10}: cannot open '{filename}' (No such file, directory or flag)",
                    filename = file
                );
                return true;
            } else {
                return false;
            }
        })
        .for_each(|file| {
            std::thread::scope(|s| {
                s.spawn(|| {
                    // print mime type
                    if show_mime_type {
                        println!("{:<10}: {:<10}", file, get_mime_type(file));
                    }
                    // default output(prints extension)
                    else if show_extension {
                        println!("{:<10}: {:<10}", file, get_file_extension(file));
                    } else {
                        println!("{:<10}: {:<10}", file, get_file_type(file));
                    }
                });
            });
        });
}

fn is_exists(filename: &str) -> bool {
    Path::new(filename).exists()
}

fn get_mime_type(filename: &str) -> String {
    match mime_guess::from_path(filename).first() {
        Some(mime) => format!("{}", mime).to_string(),
        // if mime type is not found, just show it as a plain text
        _ => {
            if get_file_type(filename) == "directory" {
                return "inode/directory".to_string();
            } else {
                return "text/plain".to_string();
            }
        }
    }
}

fn get_file_extension(filename: &str) -> String {
    match Path::new(filename).extension().and_then(OsStr::to_str) {
        Some(ext) => format!("{}", ext).to_string(),
        None => "???".to_string(),
    }
}

fn get_file_type(filename: &str) -> String {
    let metadata = fs::metadata(filename);
    let file_type = metadata.expect("Couldn't read files metadata!").file_type();
    if file_type.is_symlink() == true {
        let actual_file = fs::read_link(filename);
        return format!("symbolic link to {:?}", actual_file).to_owned();
    } else if file_type.is_block_device() == true {
        return "block special".to_owned();
    } else if file_type.is_char_device() == true {
        return "char device".to_owned();
    } else if file_type.is_fifo() == true {
        return "fifo".to_owned();
    } else if file_type.is_socket() == true {
        return "socket".to_owned();
    } else if file_type.is_dir() == true {
        return "directory".to_owned();
    } else if file_type.is_file() == true {
        return "ASCII text".to_owned();
    } else {
        return "???".to_owned();
    }
}
