#[cfg(zip)]
use zip::ZipArchive;

#[cfg(txz)]
use xz2::read::XzDecoder;

#[cfg(tgz)]
use flate2::read::GzDecoder;

#[cfg(sevenz)]
use sevenz_rust2::{ArchiveReader as ArchiveReader7z, Password};

#[cfg(alltar)]
use tar::Archive as ArchiveTar;



use std::collections::HashMap;

#[cfg(any(zip, tar, txz, txz))]
use std::io::Read;

#[cfg(alltar)]
use std::io::BufReader;


use std::path::{Path, PathBuf};

#[cfg(sevenz)]
use crate::utils::is_some;

use crate::{ Result };


#[cfg(any(sevenz, zip))]
use crate::utils::{ File, create_file };



#[derive(Clone, PartialEq, Eq)]
pub enum ArchiveType {
    #[cfg(sevenz)]
    SevenZ,
    #[cfg(zip)]
    Zip,
    #[cfg(tar)]
    Tar,
    #[cfg(tgz)]
    TarGz,
    #[cfg(tgz)]
    Tgz,
    #[cfg(txz)]
    TarXz,
    #[cfg(txz)]
    Txz,
}

impl ArchiveType {
    
    pub fn ext(self) -> &'static str {
        match self {
            #[cfg(sevenz)]
            ArchiveType::SevenZ => ".7z",
            #[cfg(zip)]
            ArchiveType::Zip => ".zip",
            #[cfg(tar)]
            ArchiveType::Tar => ".tar",
            #[cfg(tgz)]
            ArchiveType::TarGz => ".tar.gz",
            #[cfg(tgz)]
            ArchiveType::Tgz => ".tgz",
            #[cfg(txz)]
            ArchiveType::TarXz => ".tar.xz",
            #[cfg(txz)]
            ArchiveType::Txz => ".txz"
        }
    }
    
    pub fn from_ext(ext: &str) -> Option<ArchiveType> {
        match ext {
            #[cfg(sevenz)]
            ".7z" => Some(ArchiveType::SevenZ),
            #[cfg(zip)]
            ".zip" => Some(ArchiveType::Zip),
            #[cfg(tar)]
            ".tar" => Some(ArchiveType::Tar),
            #[cfg(tgz)]
            ".tar.gz" => Some(ArchiveType::TarGz),
            #[cfg(tgz)]
            ".tgz" => Some(ArchiveType::Tgz),
            #[cfg(txz)]
            ".tar.xz" => Some(ArchiveType::TarXz),
            #[cfg(txz)]
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
            #[cfg(sevenz)]
            ".7z" => Some(ArchiveType::SevenZ),
            #[cfg(zip)]
            ".zip" => Some(ArchiveType::Zip),
            #[cfg(tar)]
            ".tar" => Some(ArchiveType::Tar),
            #[cfg(tgz)]
            ".tar.gz" => Some(ArchiveType::TarGz),
            #[cfg(tgz)]
            ".tgz" => Some(ArchiveType::Tgz),
            #[cfg(txz)]
            ".tar.xz" => Some(ArchiveType::TarXz),
            #[cfg(txz)]
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
        #[cfg(sevenz)]
        ArchiveType::SevenZ => archive_7z(read, data, callback),
        #[cfg(zip)]
        ArchiveType::Zip => archive_zip(read, data, callback),
        #[cfg(tar)]
        ArchiveType::Tar => archive_tar(read, data, callback),
        #[cfg(tgz)]
        ArchiveType::TarGz => archive_tar_gz(read, data, callback),
        #[cfg(tgz)]
        ArchiveType::Tgz => archive_tar_gz(read, data, callback),
        #[cfg(txz)]
        ArchiveType::TarXz => archive_tar_xz(read, data, callback),
        #[cfg(txz)]
        ArchiveType::Txz => archive_tar_xz(read, data, callback),
    }
}



#[cfg(sevenz)]
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

#[cfg(zip)]
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

#[cfg(tar)]
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

#[cfg(tgz)]
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

#[cfg(txz)]
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