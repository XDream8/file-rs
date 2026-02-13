use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptType {
    Bash,
    Tcsh,
    Csh,
    Yash,
    Ash,
    Ksh,
    Zsh,
    Sh,
    Python,
}

impl ScriptType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Bash => "Bourne-Again shell",
            Self::Tcsh => "Tenex C shell",
            Self::Csh => "C shell",
            Self::Yash => "Yet-Another shell",
            Self::Ash => "Neil Brown's ash",
            Self::Ksh => "Korn shell",
            Self::Zsh => "Paul Falstad's zsh",
            Self::Sh => "POSIX shell",
            Self::Python => "Python",
        }
    }
}

// strange situation
// first_line is fetched as "[forbid(unsafe_code)]" when used on this file
// probably a bug in this rust version: 1.93.0
#[inline(always)]
pub fn get_type_from_shebang(path: &Path) -> Option<String> {
    // Open the file in read-only mode (if we encounter an error it returns None).
    let first_line: String = BufReader::new(File::open(path).ok()?)
        .lines()
        .next()? // Get the first line (Option<Result<String>>)
        .ok()?; // Convert Result to Option

    // return empty string if file does not have a shebang
    if !first_line.starts_with("#!") {
        return None;
    }

    // we dont want to take shebang flags if there is any
    let first_line: &str = first_line.trim_start_matches("#!").trim();

    let shebang: &str = if !first_line.contains("/env ") || first_line.ends_with("env") {
        // take the first shebang component
        first_line.split(' ').next().unwrap_or_default()
    } else {
        // take the command after env
        first_line.splitn(2, ' ').last().unwrap_or_default()
    };

    Some(evaluate_shebang(shebang))
}

#[inline(always)]
fn evaluate_shebang(shebang: &str) -> String {
    match shebang {
        _ if shebang.contains("bash") => ScriptType::Bash.as_str(),
        _ if shebang.contains("tcsh") => ScriptType::Tcsh.as_str(),
        _ if shebang.contains("csh") => ScriptType::Csh.as_str(),
        _ if shebang.contains("yash") => ScriptType::Yash.as_str(),
        _ if shebang.contains("ash") => ScriptType::Ash.as_str(),
        _ if shebang.contains("ksh") => ScriptType::Ksh.as_str(),
        _ if shebang.contains("zsh") => ScriptType::Zsh.as_str(),
        _ if shebang.contains("sh") => ScriptType::Sh.as_str(),
        _ if shebang.contains("python") => ScriptType::Python.as_str(),
        _ => shebang,
    }
    .to_owned()
}

#[cfg(test)]
pub mod tests {
    use super::evaluate_shebang;
    use crate::inside_file::ScriptType;

    #[test]
    fn test_evaluate_shebang() {
        assert_eq!(evaluate_shebang("/lorem/bash"), ScriptType::Bash.as_str());
        assert_eq!(evaluate_shebang("tcsh -ipsum"), ScriptType::Tcsh.as_str());
        assert_eq!(evaluate_shebang("export l=s csh"), ScriptType::Csh.as_str());
        assert_eq!(evaluate_shebang("amet/yash"), ScriptType::Yash.as_str());
        assert_eq!(evaluate_shebang("ash --consec"), ScriptType::Ash.as_str());
        assert_eq!(evaluate_shebang("adip; ksh"), ScriptType::Ksh.as_str());
        assert_eq!(evaluate_shebang("elit/zsh"), ScriptType::Zsh.as_str());
        assert_eq!(evaluate_shebang("sed && sh"), ScriptType::Sh.as_str());
        assert_eq!(evaluate_shebang("do python c"), ScriptType::Python.as_str());
    }

    #[test]
    fn test_bad_evaluate_shebang() {
        assert_eq!(evaluate_shebang("unknown"), "unknown");
    }
}
