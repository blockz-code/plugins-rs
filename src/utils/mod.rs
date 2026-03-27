
#[cfg(archives)]
use crate::Result;

#[cfg(archives)]
use std::io::Write;

use std::path::PathBuf;






#[cfg(archives)]
pub type File = tempfile::SpooledTempFile;

#[cfg(archives)]
pub fn create_file(buf: &[u8]) -> Result<File> {
    let mut file = tempfile::spooled_tempfile(buf.len());
    file.write_all(buf)?;
    Ok(file)
}





pub fn exts_files() -> Vec<&'static str> {
    vec![
        ".json",
        ".ts", ".tsx",
        ".js", ".mjs", ".cjs", ".jsx",
    ]
}

pub fn exts_archives() -> Vec<&'static str> {

    #[cfg(any(feature = "7z", feature = "zip", feature = "tar", feature = "tar_gz", feature = "tar_xz"))]
    let mut ext = vec![];
    #[cfg(not(any(feature = "7z", feature = "zip", feature = "tar", feature = "tar_gz", feature = "tar_xz")))]
    let ext = vec![];

    #[cfg(feature = "7z")]
    ext.extend(vec![".7z"]);

    #[cfg(feature = "zip")]
    ext.extend(vec![".zip"]);

    #[cfg(feature = "tar")]
    ext.extend(vec![".tar"]);

    #[cfg(feature = "tar_gz")]
    ext.extend(vec![".tar.gz", ".tgz"]);

    #[cfg(feature = "tar_xz")]
    ext.extend(vec![".tar.xz", ".txz"]);

    ext
}





pub fn check_for_archive(path: String) -> (bool, &'static str) {
    for ext in exts_archives() {
        if path.ends_with(ext) {
            return (true, ext);
        }
    }
    return (false, "");
}

pub fn check_for_file(path: String) -> (bool, &'static str) {
    for ext in exts_files() {
        if path.ends_with(ext) {
            return (true, ext);
        }
    }
    return (false, "");
}

pub fn is_some(path: PathBuf) -> bool {
    let mut exts = exts_files();
    exts.extend(exts_archives());
    for ext in exts {
        match path.file_name() {
            Some(name) => if name.to_str().unwrap().ends_with(ext) {
                return true;
            },
            None => return false,
        }
    }
    return false;
}
