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


#[allow(unused)]
use std::{path::PathBuf, io::{BufReader, Read}};
#[allow(unused)]
use crate::{File, Result, create_file};




pub fn archive<F>(ext: &str, read: bool, data: &[u8], callback: F) -> Result<()>
where
    F: FnMut(PathBuf, Option<Vec<u8>>) + Send + Sync,
{
    match ext {
        #[cfg(feature = "7z")]
        ".7z" => archive_7z(read, data, callback),
        #[cfg(feature = "zip")]
        ".zip" => archive_zip(read, data, callback),
        #[cfg(feature = "tar")]
        ".tar" => archive_tar(read, data, callback),
        #[cfg(feature = "tar_gz")]
        ".tar.gz" => archive_tar_gz(read, data, callback),
        #[cfg(feature = "tar_xz")]
        ".tar.xz" => archive_tar_xz(read, data, callback),
        _ => Ok(())
    }
}




#[cfg(feature = "7z")]
pub fn archive_7z<F>(read: bool, data: &[u8], mut callback: F) -> Result<()>
where
    F: FnMut(PathBuf, Option<Vec<u8>>) + Send + Sync,
{
    let mut temp: File = create_file(data)?;
    let mut archive = ArchiveReader7z::new(&mut temp, Password::empty())?;
    for entry in archive.archive().files.clone() {
        let data = if read { Some(archive.read_file(entry.name())?) } else { None };
        callback(PathBuf::from(entry.name()), data);
    }
    Ok(())
}



#[cfg(feature = "zip")]
pub fn archive_zip<F>(read: bool, data: &[u8], mut callback: F) -> Result<()>
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

        callback(PathBuf::from(entry), data);
    }
    Ok(())
}



#[cfg(feature = "tar")]
pub fn archive_tar<F>(read: bool, data: &[u8], mut callback: F) -> Result<()>
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
        callback(path.to_path_buf(), data);
    }
    Ok(())
}



#[cfg(feature = "tar_gz")]
pub fn archive_tar_gz<F>(read: bool, data: &[u8], mut callback: F) -> Result<()>
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
        callback(path.to_path_buf(), data);
    }
    Ok(())
}



#[cfg(feature = "tar_xz")]
pub fn archive_tar_xz<F>(read: bool, data: &[u8], mut callback: F) -> Result<()>
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
        callback(path.to_path_buf(), data);
    }
    Ok(())
}