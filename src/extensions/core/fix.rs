

pub fn rust_which(value: String) -> crate::Result<std::path::PathBuf> {
    Ok(which::which(value)?)
}