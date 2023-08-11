// for getting file ext
use std::{ffi::OsStr, path::Path};

// for getting file type
use std::fs::{metadata, read_link, symlink_metadata, FileType, Metadata};
use std::os::unix::fs::{FileTypeExt, MetadataExt};

#[cfg(feature = "mime_guess")]
#[inline(always)]
pub fn get_mime_type(path: &Path) -> String {
    match mime_guess::from_path(path).first() {
        Some(mime) => mime.to_string(),
        // if mime type is not found, just show it as a plain text
        _ => {
            if path.is_dir() {
                "inode/directory".to_owned()
            } else {
                "text/plain".to_owned()
            }
        }
    }
}

#[cfg(feature = "infer")]
#[inline(always)]
pub fn get_mime_type(path: &Path) -> String {
    match infer::get_from_path(path) {
        Ok(Some(mime)) => mime.to_string(),
        // if mime type is not found, just show it as a plain text
        _ => {
            if path.is_dir() {
                "inode/directory".to_owned()
            } else {
                "text/plain".to_owned()
            }
        }
    }
}

#[inline(always)]
pub fn get_file_extension(path: &Path) -> String {
    path.extension()
        .and_then(OsStr::to_str)
        .unwrap_or("???")
        .to_owned()
}

#[inline(always)]
pub fn get_file_type(path: &Path) -> String {
    let metadata: Result<Metadata, _> = metadata(path);
    let file_type: FileType = metadata
        .as_ref()
        .expect("Couldn't read files metadata!")
        .file_type();

    if file_type.is_symlink() || is_symlink(path) {
        if let Ok(actual_file) = read_link(path) {
            format!("symbolic link to {}", actual_file.display())
        } else {
            "symbolic link".to_owned()
        }
    } else if file_type.is_block_device() {
        if let Ok(metadata) = metadata {
            let (major, minor): (u64, u64) = get_device_numbers(&metadata);
            format!("block special ({}/{})", major, minor)
        } else {
            "block special".to_owned()
        }
    } else if file_type.is_char_device() {
        if let Ok(metadata) = metadata {
            let (major, minor): (u64, u64) = get_device_numbers(&metadata);
            format!("character special ({}/{})", major, minor)
        } else {
            "character special".to_owned()
        }
    } else if file_type.is_fifo() {
        "fifo".to_owned()
    } else if file_type.is_socket() {
        "socket".to_owned()
    } else if file_type.is_dir() {
        "directory".to_owned()
    } else if file_type.is_file() {
        "ASCII text".to_owned()
    } else {
        "???".to_owned()
    }
}

// sometimes .is_symlink() method does not work as expected
// this function handles that situation
#[inline]
pub fn is_symlink(path: &Path) -> bool {
    if let Ok(metadata) = symlink_metadata(path) {
        metadata.file_type().is_symlink()
    } else {
        false
    }
}

// for block and character special devices
#[inline]
pub fn get_device_numbers(metadata: &Metadata) -> (u64, u64) {
    let major: u64 = (metadata.rdev() >> 8) & 0xfff;
    let minor: u64 = metadata.rdev() & 0xff;
    (major, minor)
}

#[cfg(test)]
pub mod tests {
    use super::{get_file_extension, get_file_type, get_mime_type};
    use std::path::Path;

    #[test]
    #[cfg(feature = "mime_guess")]
    fn test_get_mime_type() {
        assert_eq!(get_mime_type(Path::new("Cargo.toml")), "text/x-toml");
        assert_eq!(get_mime_type(Path::new("Cargo.lock")), "text/plain");
        assert_eq!(get_mime_type(Path::new("LICENSE")), "text/plain");
        assert_eq!(get_mime_type(Path::new("src")), "inode/directory");
    }

    #[test]
    #[cfg(feature = "infer")]
    fn test_get_mime_type() {
        assert_eq!(get_mime_type(Path::new("Cargo.toml")), "text/plain");
        assert_eq!(get_mime_type(Path::new("Cargo.lock")), "text/plain");
        assert_eq!(get_mime_type(Path::new("LICENSE")), "text/plain");
        assert_eq!(get_mime_type(Path::new("src")), "inode/directory");
    }

    #[test]
    fn test_get_file_extension() {
        assert_eq!(get_file_extension(Path::new("Cargo.toml")), "toml");
        assert_eq!(get_file_extension(Path::new("Cargo.lock")), "lock");
    }

    #[test]
    fn test_bad_get_file_extension() {
        assert_eq!(get_file_extension(Path::new("LICENSE")), "???");
        assert_eq!(get_file_extension(Path::new("src")), "???");
    }

    #[test]
    fn test_get_file_type() {
        assert_eq!(get_file_type(Path::new("Cargo.toml")), "ASCII text");
        assert_eq!(get_file_type(Path::new("Cargo.lock")), "ASCII text");
        assert_eq!(get_file_type(Path::new("LICENSE")), "ASCII text");
        assert_eq!(get_file_type(Path::new("src")), "directory");
    }
}
