// for cli-args
use seahorse::{App, Context, Flag, FlagType};
use std::env;
use std::process::exit;

use std::collections::HashSet;

// for getting file ext
use std::{ffi::OsStr, path::Path};

// for getting file type
use std::fs::{metadata, read_link, File};
use std::os::unix::fs::FileTypeExt;

use std::io::{BufRead, BufReader};

// threading
use rayon::{scope, ThreadPoolBuilder};

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
    let mut set = HashSet::new();
    files.retain(|x| set.insert(x.clone()));

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

                let mut skip: bool = false;
                if path.exists() == false {
                    println!("{file:<15}: cannot open '{file}' (No such file, directory or flag)");
                    skip = true;
                }

                if !skip {
                    let mut shebang: String = String::new();
                    if !show_mime_type | !show_extension {
                        shebang = get_type_from_shebang(path);
                    }
                    // print mime type
                    if show_mime_type {
                        println!("{file:<15}: {:<15}", get_mime_type(path));
                    }
                    // default output(prints extension)
                    else if show_extension {
                        println!("{file:<15}: {:<15}", get_file_extension(path));
                    }
                    // if file does not have a shebang
                    else if shebang == "" {
                        println!("{file:<15}: {:<15}", get_file_type(path));
                    }
                    // if file has a shebang
                    else {
                        println!("{file:<15}: {shebang} script, {}", get_file_type(path));
                    }
                }
            });
        }
    })
}

fn get_mime_type(path: &Path) -> String {
    match mime_guess::from_path(path).first() {
        Some(mime) => format!("{}", mime).to_string(),
        // if mime type is not found, just show it as a plain text
        _ => {
            if get_file_type(path) == "directory" {
                return "inode/directory".to_string();
            } else {
                return "text/plain".to_string();
            }
        }
    }
}

fn get_file_extension(path: &Path) -> String {
    match path.extension().and_then(OsStr::to_str) {
        Some(ext) => format!("{}", ext).to_string(),
        None => "???".to_string(),
    }
}

fn get_file_type(path: &Path) -> String {
    let metadata = metadata(path);
    let file_type = metadata.expect("Couldn't read files metadata!").file_type();
    match file_type {
        _ if file_type.is_symlink() => {
            let actual_file = read_link(path);
            return format!("symbolic link to {:?}", actual_file).to_owned();
        }
        _ if file_type.is_block_device() => "block special".to_owned(),
        _ if file_type.is_char_device() => "char device".to_owned(),
        _ if file_type.is_fifo() => "fifo".to_owned(),
        _ if file_type.is_socket() => "socket".to_owned(),
        _ if file_type.is_dir() => "directory".to_owned(),
        _ if file_type.is_file() => "ASCII text".to_owned(),
        _ => "???".to_owned(),
    }
}
fn get_type_from_shebang(path: &Path) -> String {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let mut first_line = String::new();
    let _ = reader.read_line(&mut first_line);

    // return empty string if file does not have a shebang
    if !first_line.contains("#!") {
        return String::new();
    }

    let shebang: &str;

    // we dont want to take shebang flags if there is any
    let shebang_compenents: Vec<&str> = first_line
        .trim_end()
        .trim_start_matches(|c| c == '#' || c == '!')
        .splitn(2, ' ')
        .collect();
    if !first_line.contains("/env ") || first_line.ends_with("env") {
        // take the first shebang compenent
        shebang = shebang_compenents.first().unwrap();
    } else {
        // take the command after env
        shebang = shebang_compenents.last().unwrap();
    }

    match shebang {
        _ if shebang.contains("bash") => "Bourne-Again shell".to_owned(),
        _ if shebang.contains("sh") => "POSIX shell".to_owned(),
        _ if shebang.contains("python") => "Python".to_owned(),
        _ => shebang.to_owned(),
    }
}
