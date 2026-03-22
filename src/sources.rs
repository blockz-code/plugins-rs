use std::sync::Arc;
use std::path::PathBuf;

use parking_lot::RwLock;

use walkdir::WalkDir;
use include_dir::Dir;

use crate::{ FindType, Result, get_ext, get_type };


use crate::archives;




#[derive(Debug)]
pub struct SourceRes {
    pub entry: PathBuf,
    pub search: PathBuf,
    pub archive: bool,
    pub archive_ext: Option<String>,
    pub ext: String,
    pub scheme: &'static str,
    pub data: Option<Vec<u8>>,
}



#[derive(Debug, Clone)]
pub enum Source {
    Embed(Dir<'static>),
    Static(PathBuf),
}

impl Source {

    pub fn read_source<F>(self, read: bool, mut callback: F) -> Result<()>
    where 
        F: FnMut(SourceRes) + Send + Sync,
    {

        match self {
            Source::Embed(dir) => {

                for entry in dir.entries() {
                    let entry_path = entry.path().to_path_buf();
                    let typ = get_type(entry_path.clone()).unwrap();

                    match typ {
                        FindType::Folder => {

                            for dir_entry in entry.as_dir().unwrap().entries() {

                                let t = get_type(dir_entry.path().to_path_buf()).unwrap();

                                if t == FindType::Folder || t == FindType::Unknown {
                                    continue;
                                }

                                let path = dir_entry.path().to_path_buf();
                                let ext = get_ext(path.clone()).unwrap();
                                let data = dir.get_file(&path).unwrap().contents();

                                callback(SourceRes {
                                    entry: entry_path.clone(),
                                    search: path,
                                    archive: false,
                                    archive_ext: None,
                                    ext: ext.to_string(),
                                    scheme: "embed",
                                    data: if read { Some(data.to_vec()) } else { None },
                                });

                            }

                        },
                        FindType::Archive(archive_ext) => {

                            let raw_data = dir.get_file(entry.path()).unwrap().contents();

                            archives::archive(archive_ext.as_str(), read, raw_data, |path, data| {

                                let ext = get_ext(path.clone()).unwrap();

                                callback(SourceRes {
                                    entry: entry_path.clone(),
                                    search: path,
                                    archive: true,
                                    archive_ext: Some(archive_ext.clone()),
                                    ext: ext.to_string(),
                                    scheme: "embed",
                                    data: data,
                                });

                            })?;

                        },
                            _ => {},
                    }

                }

            },
            Source::Static(dir) => {

                for res_entry in std::fs::read_dir(dir)? {
                    let entry = res_entry?;
                    let entry_path = entry.path().to_path_buf();
                    let typ = get_type(entry_path.clone()).unwrap();

                    match typ {
                        FindType::Folder => {

                            for entry in WalkDir::new(&entry_path).into_iter().filter_map(|e| e.ok()) {

                                if entry.metadata()?.is_dir() {
                                    continue;
                                }

                                let filepath = entry.path().to_str().unwrap().replace(entry_path.to_str().unwrap(), "");
                                let path = PathBuf::from(filepath.strip_prefix("\\").unwrap());
                                let ext = get_ext(path.clone()).unwrap();

                                callback(SourceRes {
                                    entry: entry_path.clone().to_path_buf(),
                                    search: path,
                                    archive: false,
                                    archive_ext: None,
                                    ext: ext.to_string(),
                                    scheme: "static",
                                    data: if read { Some(std::fs::read(entry.path().to_str().unwrap())?) } else { None },
                                });

                            }

                        },
                        FindType::Archive(archive_ext) => {

                            let data = std::fs::read(&entry_path)?;

                            archives::archive(archive_ext.as_str(), read, &data, |path, data| {

                                let ext = get_ext(path.clone()).unwrap();

                                callback(SourceRes {
                                    entry: entry_path.clone(),
                                    search: path,
                                    archive: true,
                                    archive_ext: Some(archive_ext.clone()),
                                    ext: ext.to_string(),
                                    scheme: "static",
                                    data: data,
                                });

                            })?;

                        },
                        _ => {},
                    }

                }

            },
        }

        Ok(())
    }



    pub fn find(self, entry: PathBuf, search: PathBuf) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        self.read_source(true, |res| {
            if 
                let Some(entry_a) = res.entry.file_name() &&
                let Some(entry_b) = entry.file_name() &&
                let Some(search_a) = res.search.file_name() &&
                let Some(search_b) = search.file_name()
            {
                if entry_a == entry_b && search_a == search_b {
                    data = res.data.unwrap();
                }
            }
        })?;
        Ok(data)
    }



    pub fn exsits(self, entry: PathBuf, search: PathBuf) -> Result<bool> {
        let mut data = false;
        self.read_source(false, |res| {
            if 
                let Some(entry_a) = res.entry.file_name() &&
                let Some(entry_b) = entry.file_name() &&
                let Some(search_a) = res.search.file_name() &&
                let Some(search_b) = search.file_name()
            {
                if entry_a == entry_b && search_a == search_b {
                    data = true;
                }
            }
        })?;
        Ok(data)
    }



    pub fn scheme(&self) -> &str {
        match self {
            Source::Embed(_dir) => "embed",
            Source::Static(_path_buf) => "static",
        }
    }

}






#[derive(Clone)]
pub struct Sources(pub Arc<RwLock<Vec<Source>>>);

impl Sources {

    pub fn new() -> Sources {
        Sources(Arc::new(RwLock::new(Vec::new())))
    }

    pub fn set(&self, source: Source) {
        let mut guard = self.0.write();
        guard.push(source);
        drop(guard);
    }

    pub fn find(&self, source: usize, entry: PathBuf, search: PathBuf) -> Result<Vec<u8>> {
        let guard = self.0.read();
        let source = guard.get(source).unwrap();
        let data = source.clone().find(entry, search)?;
        drop(guard);
        Ok(data)
    }

}
