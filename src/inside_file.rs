use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn get_type_from_shebang(path: &Path) -> String {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let mut first_line = String::new();
    let _ = reader.read_line(&mut first_line);

    // return empty string if file does not have a shebang
    if !first_line.contains("#!") {
        return String::new();
    }

    // we dont want to take shebang flags if there is any
    let shebang_compenents: Vec<&str> = first_line
        .trim_end()
        .trim_start_matches(|c| c == '#' || c == '!')
        .splitn(2, ' ')
        .collect();
    let shebang: &str = if !first_line.contains("/env ") || first_line.ends_with("env") {
        // take the first shebang compenent
        shebang_compenents.first().unwrap()
    } else {
        // take the command after env
        shebang_compenents.last().unwrap()
    };

    evaluate_shebang(shebang).to_owned()
}

fn evaluate_shebang(shebang: &str) -> &str {
    match shebang {
        _ if shebang.contains("bash") => "Bourne-Again shell",
        _ if shebang.contains("tcsh") => "Tenex C shell",
        _ if shebang.contains("csh") => "C shell",
        _ if shebang.contains("ash") => "Neil Brown's ash",
        _ if shebang.contains("ksh") => "Korn shell",
        _ if shebang.contains("zsh") => "Paul Falstad's zsh",
        _ if shebang.contains("sh") => "POSIX shell",
        _ if shebang.contains("python") => "Python",
        _ => shebang,
    }
}

#[cfg(test)]
pub mod tests {
    use super::evaluate_shebang;

    #[test]
    fn test_evaluate_shebang() {
        assert_eq!(evaluate_shebang("bash"), "Bourne-Again shell");
        assert_eq!(evaluate_shebang("tcsh"), "Tenex C shell");
        assert_eq!(evaluate_shebang("csh"), "C shell");
        assert_eq!(evaluate_shebang("ash"), "Neil Brown's ash");
        assert_eq!(evaluate_shebang("ksh"), "Korn shell");
        assert_eq!(evaluate_shebang("zsh"), "Paul Falstad's zsh");
        assert_eq!(evaluate_shebang("sh"), "POSIX shell");
        assert_eq!(evaluate_shebang("python"), "Python");
    }

    #[test]
    fn test_bad_evaluate_shebang() {
        assert_eq!(evaluate_shebang("unknown"), "unknown");
    }
}
