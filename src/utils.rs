use crate::{Result, Error, Url};

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;

use deno_core::ModuleType;
use mime_guess2::from_ext;
use tempfile::spooled_tempfile;

use deno_ast::MediaType;
use deno_ast::ParseParams;
use deno_ast::SourceMapOption;
use deno_core::ModuleSpecifier;
use deno_error::JsErrorBox;




pub type File = tempfile::SpooledTempFile;

pub type SourceMapStore = Rc<RefCell<HashMap<String, Vec<u8>>>>;




pub fn create_file(buf: &[u8]) -> Result<File> {
    let mut file = spooled_tempfile(buf.len());
    file.write_all(buf)?;
    Ok(file)
}






#[derive(Debug, PartialEq, Eq)]
pub enum FindType {
    Folder,
    File(String),
    Archive(String),

    Unknown,
}

impl FindType {

    pub fn ext(self) -> Option<String> {
        match self {
            FindType::Folder => None,
            FindType::File(ext) => Some(ext),
            FindType::Archive(ext) => Some(ext),
            FindType::Unknown => None,
        }
    }

    pub fn mime(&self) -> Option<String> {
        match self {
            FindType::Folder => None,
            FindType::File(ext) => Some(from_ext(ext).first_raw().unwrap().to_string()),
            FindType::Archive(ext) => Some(from_ext(ext).first_raw().unwrap().to_string()),
            FindType::Unknown => None,
        }
    }

}






pub fn exts_files() -> Vec<&'static str> {
    vec![
        ".js",
        ".mjs",
        ".cjs",
        ".jsx",
        ".ts",
        ".tsx",
        ".json",
    ]
}

pub fn exts_archives() -> Vec<&'static str> {
    vec![
        #[cfg(feature = "7z")]
        ".7z",
        #[cfg(feature = "zip")]
        ".zip",
        #[cfg(feature = "tar")]
        ".tar",
        #[cfg(feature = "tar_gz")]
        ".tar.gz",
        #[cfg(feature = "tar_xz")]
        ".tar.xz",
    ]
}


pub fn is_archive(path: PathBuf) -> (bool, &'static str) {
    for ext in exts_archives() {
        if path.to_str().unwrap().ends_with(ext) {
            return (true, ext);
        }
    }
    return (false, "");
}


pub fn is_file(path: PathBuf) -> (bool, &'static str) {
    for ext in exts_files() {
        if path.to_str().unwrap().ends_with(ext) {
            return (true, ext);
        }
    }
    return (false, "");
}


pub fn is_supported(path: PathBuf) -> bool {
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


pub fn get_ext(path: PathBuf) -> Option<&'static str> {
    let mut exts = exts_files();
    exts.extend(exts_archives());
    for ext in exts {
        if path.to_str().unwrap().ends_with(ext) {
            return Some(ext);
        }
    }
    return None;
}


pub fn get_type(path: PathBuf) -> Option<FindType> {
    if is_supported(path.clone()) {
        for ext in exts_files() {
            if path.to_str().unwrap().ends_with(ext) {
                return Some(FindType::File(ext.to_string()));
            }
        }
        for ext in exts_archives() {
            if path.to_str().unwrap().ends_with(ext) {
                return Some(FindType::Archive(ext.to_string()));
            }
        }
    } else {
        return Some(FindType::Folder);
    }
    return None;
}



pub fn transpile(code: String, source_maps: SourceMapStore, module_specifier: &ModuleSpecifier, media_type: MediaType, should_transpile: bool) -> crate::Result<String> {
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



pub fn mediatype(uri: &str) -> Result<(MediaType, ModuleType, bool)> {
    let media_type = MediaType::from_path(&PathBuf::from(uri.replace("/\\", "\\")));
    match media_type {
        MediaType::JavaScript | MediaType::Mjs | MediaType::Cjs => Ok((media_type, ModuleType::JavaScript, false)),
        MediaType::Jsx => Ok((media_type, ModuleType::JavaScript, true)),
        MediaType::TypeScript => Ok((media_type, ModuleType::JavaScript, true)),
        MediaType::Tsx => Ok((media_type, ModuleType::JavaScript, true)),
        MediaType::Json => Ok((media_type, ModuleType::Json, false)),
        _ => {
            return Err(Error::Unknown(format!("Unknown extension")));
        }
    }
}






pub struct UrlInfo {
    pub scheme: String,
    pub entry: PathBuf,
    pub search: PathBuf,
    pub source: usize,
}



pub fn url_create(scheme: &str, identifier: &str, search: PathBuf, source: usize, entry: PathBuf) -> Result<Url> {
    let parts: Vec<&str> = identifier.split('.').collect();
    let id = parts.iter().rev().copied().collect::<Vec<_>>().join(".");
    let surl = format!("{}://{}:{}@{}", scheme, search.to_str().unwrap(), source, format!("{}/{}", id, entry.to_str().unwrap()));
    Ok(Url::parse(&surl)?)
}


pub fn url_info(uri: Url) -> Result<UrlInfo> {
    Ok(UrlInfo {
        scheme: uri.scheme().to_string(),
        entry: PathBuf::from(uri.path().strip_prefix("/").unwrap()),
        search: PathBuf::from(uri.username()),
        source: uri.password().unwrap().parse::<usize>().unwrap(),
    })
}
