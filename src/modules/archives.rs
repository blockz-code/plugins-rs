#[cfg(feature = "zip")]
use zip::ZipArchive;

#[cfg(feature = "tar_xz")]
use xz2::read::XzDecoder;

#[cfg(feature = "tar_gz")]
use flate2::read::GzDecoder;

#[cfg(feature = "7z")]
use sevenz_rust2::{ArchiveReader as ArchiveReader7z, Password};

#[cfg(any(feature = "tar", feature = "tar_xz", feature = "tar_gz"))]
use tar::Archive as ArchiveTar;



use std::collections::HashMap;
use std::io::{ BufReader, Read };
use std::path::{Path, PathBuf};

#[cfg(feature = "7z")]
use crate::utils::is_some;
use crate::{ Result };

use crate::utils::{ File, create_file };



#[derive(Clone, PartialEq, Eq)]
pub enum ArchiveType {
    #[cfg(feature = "7z")]
    SevenZ,
    #[cfg(feature = "zip")]
    Zip,
    #[cfg(feature = "tar")]
    Tar,
    #[cfg(feature = "tar_gz")]
    TarGz,
    #[cfg(feature = "tar_gz")]
    Tgz,
    #[cfg(feature = "tar_xz")]
    TarXz,
    #[cfg(feature = "tar_xz")]
    Txz,
}

impl ArchiveType {
    
    pub fn ext(self) -> &'static str {
        match self {
            #[cfg(feature = "7z")]
            ArchiveType::SevenZ => ".7z",
            #[cfg(feature = "zip")]
            ArchiveType::Zip => ".zip",
            #[cfg(feature = "tar")]
            ArchiveType::Tar => ".tar",
            #[cfg(feature = "tar_gz")]
            ArchiveType::TarGz => ".tar.gz",
            #[cfg(feature = "tar_gz")]
            ArchiveType::Tgz => ".tgz",
            #[cfg(feature = "tar_xz")]
            ArchiveType::TarXz => ".tar.xz",
            #[cfg(feature = "tar_xz")]
            ArchiveType::Txz => ".txz"
        }
    }
    
    pub fn from_ext(ext: &str) -> Option<ArchiveType> {
        match ext {
            #[cfg(feature = "7z")]
            ".7z" => Some(ArchiveType::SevenZ),
            #[cfg(feature = "zip")]
            ".zip" => Some(ArchiveType::Zip),
            #[cfg(feature = "tar")]
            ".tar" => Some(ArchiveType::Tar),
            #[cfg(feature = "tar_gz")]
            ".tar.gz" => Some(ArchiveType::TarGz),
            #[cfg(feature = "tar_gz")]
            ".tgz" => Some(ArchiveType::Tgz),
            #[cfg(feature = "tar_xz")]
            ".tar.xz" => Some(ArchiveType::TarXz),
            #[cfg(feature = "tar_xz")]
            ".txz" => Some(ArchiveType::Txz),
            _ => None
        }
    }
    
    pub fn from_path<P: AsRef<Path>>(path: P) -> Option<ArchiveType> {

        let extension = path
            .as_ref()
            .extension()
            .map(|s| format!(".{}", s.display().to_string()))
            .unwrap();

        match extension.as_str() {
            #[cfg(feature = "7z")]
            ".7z" => Some(ArchiveType::SevenZ),
            #[cfg(feature = "zip")]
            ".zip" => Some(ArchiveType::Zip),
            #[cfg(feature = "tar")]
            ".tar" => Some(ArchiveType::Tar),
            #[cfg(feature = "tar_gz")]
            ".tar.gz" => Some(ArchiveType::TarGz),
            #[cfg(feature = "tar_gz")]
            ".tgz" => Some(ArchiveType::Tgz),
            #[cfg(feature = "tar_xz")]
            ".tar.xz" => Some(ArchiveType::TarXz),
            #[cfg(feature = "tar_xz")]
            ".txz" => Some(ArchiveType::Txz),
            _ => None
        }
    }

}



#[allow(unused)]
pub fn get_archive_files(ext: ArchiveType, data: &[u8]) -> Result<HashMap<PathBuf, Vec<u8>>> {
    let mut map = HashMap::new();
    read_archive(ext, true, data, | path, content | {
        map.insert(path, content.unwrap());
    })?;
    Ok(map)
}



pub fn read_archive<F>(ext: ArchiveType, read: bool, data: &[u8], callback: F) -> Result<()>
where
    F: FnMut(PathBuf, Option<Vec<u8>>) + Send + Sync,
{
    match ext {
        #[cfg(feature = "7z")]
        ArchiveType::SevenZ => archive_7z(read, data, callback),
        #[cfg(feature = "zip")]
        ArchiveType::Zip => archive_zip(read, data, callback),
        #[cfg(feature = "tar")]
        ArchiveType::Tar => archive_tar(read, data, callback),
        #[cfg(feature = "tar_gz")]
        ArchiveType::TarGz => archive_tar_gz(read, data, callback),
        #[cfg(feature = "tar_gz")]
        ArchiveType::Tgz => archive_tar_gz(read, data, callback),
        #[cfg(feature = "tar_xz")]
        ArchiveType::TarXz => archive_tar_xz(read, data, callback),
        #[cfg(feature = "tar_xz")]
        ArchiveType::Txz => archive_tar_xz(read, data, callback),
    }
}



#[cfg(feature = "7z")]
fn archive_7z<F>(read: bool, data: &[u8], mut callback: F) -> Result<()>
where
    F: FnMut(PathBuf, Option<Vec<u8>>) + Send + Sync,
{
    let mut temp: File = create_file(data)?;
    let mut archive = ArchiveReader7z::new(&mut temp, Password::empty())?;
    for entry in archive.archive().files.clone() {
        let data = if read { Some(archive.read_file(entry.name())?) } else { None };
        let path = PathBuf::from(entry.name().to_string());
        if is_some(path.clone()) {
            callback(path, data);
        }
    }
    Ok(())
}

#[cfg(feature = "zip")]
fn archive_zip<F>(read: bool, data: &[u8], mut callback: F) -> Result<()>
where
    F: FnMut(PathBuf, Option<Vec<u8>>) + Send + Sync,
{
    let temp: File = create_file(data)?;
    let mut archive = ZipArchive::new(temp)?;
    let mut entries = vec![];
    for entry in archive.file_names() {
        entries.push(entry.to_string());
    }
    for entry in entries {
        let mut reader = archive.by_path(&entry)?;
        let data = if read {
            let mut data = Vec::new();
            data.resize(reader.size() as usize, 0);
            reader.read_exact(&mut data)?;
            Some(data.to_vec())
        } else { None };
        let path = PathBuf::from(entry);
        if is_some(path.clone()) {
            callback(path, data);
        }
    }
    Ok(())
}

#[cfg(feature = "tar")]
fn archive_tar<F>(read: bool, data: &[u8], mut callback: F) -> Result<()>
where
    F: FnMut(PathBuf, Option<Vec<u8>>) + Send + Sync,
{
    for entry0 in ArchiveTar::new(BufReader::new(data)).entries()? {
        let mut entry = entry0?;
        let data = if read {
            let mut data = Vec::new();
            data.resize(entry.size() as usize, 0);
            entry.read(&mut data)?;
            Some(data.to_vec())
        } else { None };
        let path = entry.path()?;
        let path = path.strip_prefix("./")?;
        if path.to_str().unwrap().len() == 0 {
            continue;
        }
        if is_some(path.to_path_buf()) {
            callback(path.to_path_buf(), data);
        }
    }
    Ok(())
}

#[cfg(feature = "tar_gz")]
fn archive_tar_gz<F>(read: bool, data: &[u8], mut callback: F) -> Result<()>
where
    F: FnMut(PathBuf, Option<Vec<u8>>) + Send + Sync,
{
    for entry in ArchiveTar::new(GzDecoder::new(BufReader::new(data))).entries()? {
        let mut entry = entry?;
        let data = if read {
            let mut data = Vec::new();
            data.resize(entry.size() as usize, 0);
            entry.read(&mut data)?;
            Some(data.to_vec())
        } else { None };
        let path = entry.path()?;
        let path = path.strip_prefix("./")?;
        if path.to_str().unwrap().len() == 0 {
            continue;
        }
        if is_some(path.to_path_buf()) {
            callback(path.to_path_buf(), data);
        }
    }
    Ok(())
}

#[cfg(feature = "tar_xz")]
fn archive_tar_xz<F>(read: bool, data: &[u8], mut callback: F) -> Result<()>
where
    F: FnMut(PathBuf, Option<Vec<u8>>) + Send + Sync,
{
    for entry in ArchiveTar::new(XzDecoder::new(BufReader::new(data))).entries()? {
        let mut entry = entry?;
        let data = if read {
            let mut data = Vec::new();
            data.resize(entry.size() as usize, 0);
            entry.read(&mut data)?;
            Some(data.to_vec())
        } else { None };
        let path = entry.path()?;
        let path = path.strip_prefix("./")?;
        if path.to_str().unwrap().len() == 0 {
            continue;
        }
        if is_some(path.to_path_buf()) {
            callback(path.to_path_buf(), data);
        }
    }
    Ok(())
}