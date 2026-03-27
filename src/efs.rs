use crate::Result;

use std::path::Path;
use std::path::PathBuf;


#[derive(Debug, Clone)]
pub enum EFsPath {
    Embed(&'static str),
    Local(PathBuf),
}

impl EFsPath {
    
    pub fn to_str(&self) -> &str {
        match &self {
            EFsPath::Embed(str) => str,
            EFsPath::Local(str) => str.to_str().unwrap(),
        }
    }
                
    pub fn to_path_buf(&self) -> PathBuf {
        Path::new(self.to_str()).to_path_buf()
    }
                
    pub fn file_name(&self) -> Option<&str> {
        Path::new(self.to_str()).file_name().and_then(|n| n.to_str())
    }
                
    pub fn ext(&self) -> Option<&str> {
        Path::new(self.to_str()).extension().and_then(|e| e.to_str())
    }
    
    pub fn depth(&self) -> usize {
        self.to_str().strip_suffix(&self.file_name().unwrap()).unwrap().split("/").collect::<Vec<&str>>().len()
    }

}

#[derive(Debug, Clone)]
pub enum EFsData {
    Embed(&'static [u8]),
    Local(Vec<u8>),
}

impl EFsData {
    
    pub fn to_str(&self) -> Option<&str> {
        match &self {
            EFsData::Embed(items) => std::str::from_utf8(items).ok(),
            EFsData::Local(items) => std::str::from_utf8(&items).ok(),
        }
    }
    
    pub fn data(&self) -> &[u8] {
        match &self {
            EFsData::Embed(items) => items,
            EFsData::Local(items) => items,
        }
    }
    
    pub fn len(&self) -> usize {
        self.data().len()
    }

}


#[derive(Debug, Clone)]
pub struct FsFile {
    pub path: EFsPath,
    pub content: EFsData,
    pub size: u64,
    pub created: u64,
    pub accessed: u64,
    pub modified: u64,
    pub depth: usize,
}










#[derive(Debug, Clone)]
pub struct PluginsFs {
    pub total: u64,
    pub files: Vec<FsFile>,
}



impl PluginsFs {

    pub fn walk<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut s = PluginsFs { ..Default::default() };
        s.walker(path)?;
        Ok(s)
    }



    pub fn size(&self) -> u64 {
        self.total
    }
                
    pub fn entries(&self) -> &[FsFile] {
        &self.files
    }
                
    pub fn count(&self) -> usize {
        self.files.len()
    }
                
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }


                
    pub fn get(&self, path: &str) -> Option<&FsFile> {
        self.files.iter().find(|f| f.path.to_str() == path)
    }
                
    pub fn as_str(&self, path: &str) -> Option<&str> {
        self.get(path).and_then(|f| f.content.to_str())
    }
                
    pub fn exists(&self, path: &str) -> bool {
        self.files.iter().any(|f| f.path.to_str() == path)
    }
                
    pub fn from_ext(&self, ext: &str) -> Vec<&FsFile> {
        self.files.iter().filter(|f| f.path.ext() == Some(ext)).collect()
    }
                
    pub fn iter(&self) -> std::slice::Iter<'_, FsFile> {
        self.files.iter()
    }

}


impl PluginsFs {
    
    fn walker<P>(&mut self, path: P) -> std::io::Result<()>
    where 
        P: AsRef<Path>,
    {
        let read_dir = std::fs::read_dir(path)?;

        for dir_entry in read_dir.into_iter().filter_map(|e| e.ok()) {

            if dir_entry.metadata()?.is_dir() {
                self.walker(dir_entry.path())?;
                continue;
            }

            let path = dir_entry.path().display().to_string().replace('\\', "/");

            let efs_path = EFsPath::Local(path.into());
            let efs_data = EFsData::Local(std::fs::read(efs_path.to_str())?);
            let size = efs_data.len() as u64;
            let depth = efs_path.depth();

            let meta = efs_path.to_path_buf().metadata()?;

            let epoch = std::time::UNIX_EPOCH;
            
            self.files.push(FsFile {
                    path: efs_path,
                    content: efs_data,
                    size: size,
                    created: meta.created()?.duration_since(epoch).unwrap().as_secs(),
                    accessed: meta.accessed()?.duration_since(epoch).unwrap().as_secs(),
                    modified: meta.modified()?.duration_since(epoch).unwrap().as_secs(),
                    depth: depth
            });

            self.total += size;
            
        }
        Ok(())
    }

}



impl Default for PluginsFs {
    fn default() -> Self {
        Self {
            total: 0,
            files: vec![],
        }
    }
}




impl std::ops::Index<&str> for PluginsFs {
    type Output = FsFile;
    fn index(&self, index: &str) -> &Self::Output {
        self.get(index).expect("file not found")
    }
}




impl<'a> IntoIterator for &'a PluginsFs {
    type Item = &'a FsFile;
    type IntoIter = std::slice::Iter<'a, FsFile>;
    fn into_iter(self) -> Self::IntoIter {
        self.files.iter()
    }
}
