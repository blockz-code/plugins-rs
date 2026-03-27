use crate::{ Result, Error };
use crate::utils::{ is_some, exts_files, exts_archives, check_for_file, check_for_archive };


use std::rc::Rc;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::collections::HashMap;



use deno_ast::MediaType;
use deno_core::ModuleType;
use deno_ast::ParseParams;
use deno_ast::SourceMapOption;
use deno_core::ModuleSpecifier;
use deno_error::JsErrorBox;
use deno_core::url::Url;





pub type SourceMaps = Rc<RefCell<HashMap<String, Vec<u8>>>>;



pub fn transpile(code: String, source_maps: SourceMaps, module_specifier: &ModuleSpecifier, media_type: MediaType, should_transpile: bool) -> crate::Result<String> {
    if should_transpile {
        let parsed = deno_ast::parse_module(ParseParams {
            specifier: module_specifier.clone(),
            text: code.into(),
            media_type,
            capture_tokens: false,
            scope_analysis: false,
            maybe_syntax: None,
        }).map_err(JsErrorBox::from_err)?;

        let res = parsed
            .transpile(
                &deno_ast::TranspileOptions {
                    imports_not_used_as_values: deno_ast::ImportsNotUsedAsValues::Remove,
                    //use_decorators_proposal: true,
                    ..Default::default()
                },
                &deno_ast::TranspileModuleOptions { module_kind: None },
                &deno_ast::EmitOptions {
                    source_map: SourceMapOption::Separate,
                    inline_sources: true,
                    ..Default::default()
                },
            ).map_err(JsErrorBox::from_err)?;

        let res = res.into_source();
        let source_map = res.source_map.unwrap().into_bytes();

        source_maps.borrow_mut().insert(module_specifier.to_string(), source_map);

        Ok(res.text)
    } else {
        Ok(code)
    }
}








#[derive(Debug)]
pub struct MediaInfo {
    pub media_type  : MediaType,
    pub module_type : ModuleType,
    pub transpile   : bool
}

impl MediaInfo {
    pub fn from_path(uri: PathBuf) -> Result<Self> {

        let media_type = MediaType::from_path(&uri);

        let (module_type, transpile) = match media_type {
            
            MediaType::JavaScript => Ok((ModuleType::JavaScript, false)),
            MediaType::Mjs => Ok((ModuleType::JavaScript, false)),
            MediaType::Cjs => Ok((ModuleType::JavaScript, false)),

            MediaType::Jsx => Ok((ModuleType::JavaScript, true)),

            MediaType::TypeScript => Ok((ModuleType::JavaScript, true)),
            MediaType::Tsx => Ok((ModuleType::JavaScript, true)),

            MediaType::Json => Ok((ModuleType::Json, false)),

            _ => Err(Error::Unknown(format!("Unknown extension")))
            
        }?;

        Ok(Self { media_type, module_type, transpile })

    }
}








#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
pub enum EntryType {
    Folder,
    File(String),
    Archive(String),
    Unknown,
}

#[allow(unused)]
impl EntryType {

    pub fn ext(self) -> Option<String> {
        match self {
            EntryType::Folder => None,
            EntryType::File(ext) => Some(ext),
            EntryType::Archive(ext) => Some(ext),
            EntryType::Unknown => None,
        }
    }

    pub fn from_path(path: PathBuf) -> EntryType {
        if is_some(path.clone()) {
            for ext in exts_files() {
                if path.to_str().unwrap().ends_with(ext) {
                    return EntryType::File(ext.to_string());
                }
            }
            for ext in exts_archives() {
                if path.to_str().unwrap().ends_with(ext) {
                    return EntryType::Archive(ext.to_string());
                }
            }
        } else {
            return EntryType::Folder;
        }
        return EntryType::Unknown;
    }

    pub fn from_str(path: &str) -> EntryType {
        if is_some(path.into()) {
            for ext in exts_files() {
                if path.ends_with(ext) {
                    return EntryType::File(ext.to_string());
                }
            }
            for ext in exts_archives() {
                if path.ends_with(ext) {
                    return EntryType::Archive(ext.to_string());
                }
            }
        } else {
            return EntryType::Folder;
        }
        return EntryType::Unknown;
    }

}










#[derive(Debug)]
pub struct FileInfo {
    pub scheme: String,
    pub entry : String,
    pub search: String,
    pub source: usize,

    #[allow(unused)]
    pub plugin: String,
}

impl FileInfo {

    pub fn url_create(scheme: &str, source: usize, plugin: &str, entry: &str, search: &str) -> Url {
        Url::parse(format!("{}+{}://{}:{}@assets.serve/{}", scheme, source, search, plugin, entry).as_str()).unwrap()
    }

    pub fn from_url(uri: &Url) -> Result<FileInfo> {
        let mut scheme_parts = uri.scheme().split("+");
        let scheme = scheme_parts.next().unwrap().to_string();
        let source = scheme_parts.next().unwrap().parse::<usize>().unwrap();

        let mut entry = uri.path().strip_prefix("/").unwrap().to_string();

        let (entry_path, search) = {

            if check_for_file(entry.clone()).0 {

                let name = Path::new(&entry).file_name().unwrap().display().to_string();
                entry = entry.strip_suffix(&name).unwrap().to_string();

                (entry, name)

            } else if check_for_archive(entry.clone()).0 {

                (entry, uri.username().to_string())

            } else {

                if entry.starts_with("/") {
                    entry = entry.strip_prefix("/").unwrap().to_string();
                }
                if !entry.ends_with("/") {
                    entry = format!("{}/", entry);
                }

                (entry, uri.username().to_string())

            }
        };

        

        Ok(FileInfo {
            search: search,
            entry : entry_path,
            scheme: scheme,
            source: source,
            plugin: uri.password().map(|s| s.to_string()).unwrap_or("".into()),
        })
    }

    #[allow(unused)]
    pub fn create(&self) -> Url {
        Url::parse(format!("{}://{}:{}@assets.serve/{}", self.scheme, self.search, self.source, self.entry).as_str()).unwrap()
    }

    #[allow(unused)]
    pub fn info(&self) -> &FileInfo {
        self
    }

}