
use std::sync::Arc;
use std::path::PathBuf;

use crate::efs::{PluginsFs, FsFile};

use parking_lot::RwLock;

use crate::{Error, Result};

use crate::modules::FileInfo;

use crate::utils::is_some;
use crate::utils::check_for_file;
use crate::utils::check_for_archive;

use super::archives::{ ArchiveType, read_archive };
use super::types::{ SourceItem, SourceEntry, EmbedEntry, Plugin };



#[derive(Clone, PartialEq, Eq)]
pub enum SourceType {
    Folder,
    File(String),
    Archive(ArchiveType),
}

impl SourceType {

    pub fn get(path: &str) -> Option<SourceType> {

        let (is_file, ext_file) = check_for_file(path.to_string());
        if is_file {
            return Some(Self::File(ext_file.to_string()))
        }

        let (is_archive, ext_archive) = check_for_archive(path.to_string());
        if is_archive {
            match ArchiveType::from_ext(ext_archive) {
                Some(ext) => {
                    return Some(Self::Archive(ext))
                },
                _ => {},
            }
        }
        
        if PathBuf::from(path).metadata().ok()?.is_dir() {
            return Some(Self::Folder);
        }

        None

    }

    pub fn check(&self, source_type: &SourceType) -> bool {
        if self == source_type {
            return true
        }
        false
    }

}










pub struct Source {
    inner: SourceEntry
}

impl Source {

    pub fn from_embed(dir: PluginsFs) -> Self {
        Self {
            inner : SourceEntry::Embed(dir)
        }
    }

    pub fn from_static(dir: PathBuf) -> Self {
        Self {
            inner : SourceEntry::Static(dir)
        }
    }

    fn read_embed_all<F>(&self, source_id: usize, callback: &mut F) -> Result<()>
    where
        F: FnMut(SourceItem) + Send + Sync,
    {
        match &self.inner {
            SourceEntry::Embed(efs) => {
                for entry in efs.iter() {
                    self.get_all_files(source_id, "embed", entry, callback)?;
                }
            },
            _ => {}
        }
        Ok(())
    }

    fn read_embed_files<F>(&self, source_id: usize, callback: &mut F) -> Result<()>
    where
        F: FnMut(SourceItem) + Send + Sync,
    {
        match &self.inner {
            SourceEntry::Embed(efs) => {
                for entry in efs.iter() {
                    self.get_files(source_id, "embed", entry, callback)?;
                }
            },
            _ => {}
        }
        Ok(())
    }

    fn read_embed_archives<F>(&self, source_id: usize, callback: &mut F) -> Result<()>
    where
        F: FnMut(SourceItem) + Send + Sync,
    {
        match &self.inner {
            SourceEntry::Embed(efs) => {
                for entry in efs.iter() {
                    self.get_archive_files(source_id, "embed", entry, callback)?;
                }
            },
            _ => {}
        }
        Ok(())
    }

    fn read_all<F>(&self, source_id: usize, callback: &mut F) -> Result<()>
    where
        F: FnMut(SourceItem) + Send + Sync,
    {
        match &self.inner {
            SourceEntry::Static(path) => {
                for entry in PluginsFs::walk(path)?.iter() {
                    self.get_all_files(source_id, "static", entry, callback)?;
                }
            },
            _ => {}
        }
        Ok(())
    }

    fn read_files<F>(&self, source_id: usize, callback: &mut F) -> Result<()>
    where
        F: FnMut(SourceItem) + Send + Sync,
    {
        match &self.inner {
            SourceEntry::Static(path) => {
                for entry in PluginsFs::walk(path)?.iter() {
                    self.get_files(source_id, "static", entry, callback)?;
                }
            },
            _ => {}
        }
        Ok(())
    }

    fn read_archives<F>(&self, source_id: usize, callback: &mut F) -> Result<()>
    where
        F: FnMut(SourceItem) + Send + Sync,
    {
        match &self.inner {
            SourceEntry::Static(path) => {
                for entry in PluginsFs::walk(path)?.iter() {
                    self.get_archive_files(source_id, "static", entry, callback)?;
                }
            },
            _ => {}
        }
        Ok(())
    }

    fn get_all_files<F>(&self, source_id: usize, scheme: &str, entry: &FsFile, callback: &mut F) -> Result<()>
    where
        F: FnMut(SourceItem) + Send + Sync,
    {
        match SourceType::get(&entry.path.to_str()).unwrap() {
            SourceType::File(_ext) => {
                let filename = entry.path.file_name().unwrap();
                let filepath = entry.path.to_str().strip_suffix(&filename).unwrap();
                callback(SourceItem {
                    source: source_id,
                    scheme: scheme.to_string(),
                    entry: filepath.to_string(),
                    search: filename.to_string(),
                    archive: None,
                    content: Some(entry.content.data().to_vec()),
                });
            },
            SourceType::Archive(atype) => read_archive(atype, true, entry.content.data(), |apath, adata| {
                if is_some(apath.clone()) {
                    callback(SourceItem {
                        source: source_id,
                        scheme: format!("{}-archive", scheme),
                        entry: entry.path.to_str().to_string(),
                        search: apath.display().to_string(),
                        archive: Some(entry.path.to_str().to_string()),
                        content: adata,
                    });
                }
            })?,
            _ => {}
        } 
        Ok(())
    }

    fn get_files<F>(&self, source_id: usize, scheme: &str, entry: &FsFile, callback: &mut F) -> Result<()>
    where
        F: FnMut(SourceItem) + Send + Sync,
    {
        match SourceType::get(&entry.path.to_str()).unwrap() {
            SourceType::File(_ext) => {
                let filename = entry.path.file_name().unwrap();
                let filepath = entry.path.to_str().strip_suffix(&filename).unwrap();
                callback(SourceItem {
                    source: source_id,
                    scheme: scheme.to_string(),
                    entry: filepath.to_string(),
                    search: filename.to_string(),
                    archive: None,
                    content: Some(entry.content.data().to_vec()),
                });
            },
            _ => {}
        } 
        Ok(())
    }

    fn get_archive_files<F>(&self, source_id: usize, scheme: &str, entry: &FsFile, callback: &mut F) -> Result<()>
    where
        F: FnMut(SourceItem) + Send + Sync,
    {
        match SourceType::get(&entry.path.to_str()).unwrap() {
            SourceType::Archive(atype) => read_archive(atype, true, entry.content.data(), |apath, adata| {
                if is_some(apath.clone()) {
                    callback(SourceItem {
                        source: source_id,
                        scheme: format!("{}-archive", scheme),
                        entry: entry.path.to_str().to_string(),
                        search: apath.display().to_string(),
                        archive: Some(entry.path.to_str().to_string()),
                        content: adata,
                    });
                }
            })?,
            _ => {}
        } 
        Ok(())
    }


}












pub struct SourceHolder {
    pub sources: Arc<RwLock<Vec<Source>>>,
    pub embedded: Arc<RwLock<Vec<Source>>>,
}


pub struct LoadedEntrys {
    pub plugins: Arc<RwLock<Vec<Plugin>>>,
    pub embedded: Arc<RwLock<Vec<EmbedEntry>>>,
}


pub struct SourceLoader {
    pub entry: String,
    pub plugin: String,
    pub holder: SourceHolder,
    pub loaded: LoadedEntrys,
}

impl SourceLoader {

    pub fn new() -> Self {
        Self {
            entry: String::from("entry.ts"),
            plugin: String::from("plugin.json"),
            holder: SourceHolder {
                embedded: Arc::new(RwLock::new(Vec::new())),
                sources: Arc::new(RwLock::new(Vec::new())),
            },
            loaded: LoadedEntrys {
                plugins: Arc::new(RwLock::new(Vec::new())),
                embedded: Arc::new(RwLock::new(Vec::new())),
            },
        }
    }

    pub fn add(&self, source: Source) {
        let mut sources = self.holder.sources.write();
        sources.push(source);
        drop(sources);
    }

    pub fn add_embed(&self, source: Source) {
        let mut embedded = self.holder.embedded.write();
        embedded.push(source);
        drop(embedded);
    }

    pub fn preload(&self) -> Result<()> {
        let embedded = self.holder.embedded.read();
        for (num, source) in embedded.iter().enumerate() {
            source.read_embed_all(num, &mut |item| {
                let mut loaded = self.loaded.embedded.write();
                if item.search.ends_with(&self.entry) {
                    let entry = if item.archive.is_some() { item.archive.clone().unwrap() } else { item.entry };
                    loaded.push(EmbedEntry {
                        url : Some(FileInfo::url_create(&item.scheme, num, "", &entry, &item.search)),
                        source: num,
                        scheme: item.scheme.to_string(),
                        entry: entry,
                        search: item.search,
                    });
                }
                drop(loaded);
            })?;
        }
        drop(embedded);
        Ok(())
    }

    pub fn plugins(&self) -> Result<()> {
        let sources = self.holder.sources.read();
        for (num, source) in sources.iter().enumerate() {
            source.read_all(num, &mut |item| {
                let mut loaded = self.loaded.plugins.write();
                if item.search.ends_with(&self.plugin) {
                    let mut json: Plugin = serde_json::from_slice(&item.content.unwrap()).unwrap();
                    let entry = if item.archive.is_some() { item.archive.unwrap() } else { item.entry };
                    json.url = Some(FileInfo::url_create(&item.scheme, num, &json.name, &entry, &json.entry));
                    json.source = Some(item.source);
                    json.base = Some(entry);
                    loaded.push(json);
                }
                drop(loaded);
            })?;
        }
        drop(sources);
        Ok(())
    }

    pub async fn init_loaded(&self, runtime: &mut deno_core::JsRuntime) -> Result<()> {

        let embedded = self.loaded.embedded.read();
        for (_num, source) in embedded.iter().enumerate() {
            match &source.url {
                Some(uri) => {
                    let mod_id = runtime.load_side_es_module(uri).await?;
                    let result = runtime.mod_evaluate(mod_id);
                    result.await?; 
                },
                _ => {},
            }
        }
        drop(embedded);

        let plugins = self.loaded.plugins.read();
        for (_num, source) in plugins.iter().enumerate() {
            match &source.url {
                Some(uri) => {
                    let mod_id = runtime.load_side_es_module(uri).await?;
                    let result = runtime.mod_evaluate(mod_id);
                    result.await?; 
                },
                _ => {},
            }
        }
        drop(plugins);

        Ok(())
    }

}



impl SourceLoader {

    pub fn elookup(&self, fileinfo: FileInfo) -> Result<Vec<u8>> {
        let embedded = self.holder.embedded.read();
        let mut data = None;
        for (num, source) in embedded.iter().enumerate() {
            source.read_embed_files(num, &mut |item| {
                if item.entry == fileinfo.entry && item.search == fileinfo.search {
                    data = item.content;
                }
            })?;
            if data.is_some() {
                break;
            }
        }
        drop(embedded);
        match data {
            Some(data) => Ok(data),
            None => Err(Error::Unknown("data is empty so maybe no file found.".into())),
        }
    }

    pub fn lookup(&self, fileinfo: FileInfo) -> Result<Vec<u8>> {
        let sources = self.holder.sources.read();
        let mut data = None;
        for (num, source) in sources.iter().enumerate() {
            source.read_files(num, &mut |item| {
                if item.entry == fileinfo.entry && item.search == fileinfo.search {
                    data = item.content;
                }
            })?;
            if data.is_some() {
                break;
            }
        }
        drop(sources);
        match data {
            Some(data) => Ok(data),
            None => Err(Error::Unknown("data is empty so maybe no file found.".into())),
        }
    }

    pub fn ealookup(&self, fileinfo: FileInfo) -> Result<Vec<u8>> {
        let embedded = self.holder.embedded.read();
        let mut data = None;
        for (num, source) in embedded.iter().enumerate() {
            source.read_embed_archives(num, &mut |item| {
                if item.entry == fileinfo.entry && item.search == fileinfo.search {
                    data = item.content;
                }
            })?;
            if data.is_some() {
                break;
            }
        }
        drop(embedded);
        match data {
            Some(data) => Ok(data),
            None => Err(Error::Unknown("data is empty so maybe no file found.".into())),
        }
    }

    pub fn alookup(&self, fileinfo: FileInfo) -> Result<Vec<u8>> {
        let sources = self.holder.sources.read();
        let mut data = None;
        for (num, source) in sources.iter().enumerate() {
            source.read_archives(num, &mut |item| {
                if item.entry == fileinfo.entry && item.search == fileinfo.search {
                    data = item.content;
                }
            })?;
            if data.is_some() {
                break;
            }
        }
        drop(sources);
        match data {
            Some(data) => Ok(data),
            None => Err(Error::Unknown("data is empty so maybe no file found.".into())),
        }
    }

}