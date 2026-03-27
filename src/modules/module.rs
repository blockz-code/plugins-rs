use std::path::PathBuf;
use std::sync::Arc;
use std::{cell::RefCell, collections::HashMap, rc::Rc};



use deno_core::url::Url;
use deno_core::ModuleLoadOptions;
use deno_core::ModuleLoadReferrer;
use deno_core::ModuleLoadResponse;
use deno_core::ModuleSource;
use deno_core::ModuleSourceCode;
use deno_core::ModuleSpecifier;
use deno_core::ResolutionKind;
use deno_core::error::ModuleLoaderError;
use deno_error::JsErrorBox;



use super::utils::{ FileInfo, MediaInfo, SourceMaps, transpile };

use super::SourceLoader;


fn extract_plugin_name(input: &str) -> Option<(String, &str)> {
    let (at_part, next_part) = input.split_once('/')?;
    if at_part.starts_with('@') { Some((at_part.replace("@", ""), next_part)) } else { None }
}


pub struct ModuleLoader {
    pub loader: Arc<SourceLoader>,
    pub source_maps: SourceMaps,
}

impl ModuleLoader {

    pub fn new(loader: Arc<SourceLoader>) -> Self {
        Self {
            loader,
            source_maps: Rc::new(RefCell::new(HashMap::new())),
        }
    }

}

impl deno_core::ModuleLoader for ModuleLoader {

    fn resolve(&self, specifier: &str, referrer: &str, _kind: ResolutionKind) -> std::result::Result<ModuleSpecifier, ModuleLoaderError> {
        let (spec, refer) = match extract_plugin_name(specifier) {
            Some((name, path)) => {
                let mut uri = Url::parse(referrer).unwrap();
                uri.set_host(Some(&format!("plugin_name.{}", name))).unwrap();
                (path, uri.to_string())
            },
            None => (specifier, referrer.to_string()),
        };
        deno_core::resolve_import(spec, &refer).map_err(JsErrorBox::from_err)
    }

    fn load(&self, module_specifier: &ModuleSpecifier, _maybe_referrer: Option<&ModuleLoadReferrer>, _options: ModuleLoadOptions) -> ModuleLoadResponse {

        let source_maps = self.source_maps.clone();

        fn load(
            loader: &SourceLoader,
            source_maps: SourceMaps,
            module_specifier: &ModuleSpecifier,
        ) -> std::result::Result<ModuleSource, ModuleLoaderError> {

            let fileinfo = FileInfo::from_url(module_specifier).map_err(|e| JsErrorBox::generic(e.to_string()))?;
            let mediainfo = MediaInfo::from_path(PathBuf::from(fileinfo.search.clone())).map_err(|e| JsErrorBox::generic(e.to_string()))?;

            let data = match fileinfo.scheme.as_str() {
                "embed" => loader.elookup(fileinfo),
                "static" => loader.lookup(fileinfo),
                "embed-archive" => loader.ealookup(fileinfo),
                "static-archive" => loader.alookup(fileinfo),
                //"jsr" => {} comming soon
                //"npm" => {} comming soon
                //"git" => {} comming soon
                scheme => unimplemented!("{}", format!("scheme [{}] not implemented.", scheme))
            }.map_err(|e| JsErrorBox::generic(e.to_string()))?;

            let code = transpile(
                String::from_utf8_lossy(&data).to_string(), 
                source_maps, 
                module_specifier, 
                mediainfo.media_type, 
                mediainfo.transpile
            ).map_err(|e| JsErrorBox::generic(e.to_string()))?;
            
            Ok(ModuleSource::new(mediainfo.module_type, ModuleSourceCode::String(code.into()), module_specifier, None))

        }

        ModuleLoadResponse::Sync(load(&self.loader, source_maps, module_specifier))

    }

    fn get_source_map(&self, specifier: &str) -> Option<std::borrow::Cow<'_, [u8]>> {
        self.source_maps.borrow().get(specifier).map(|v| v.clone().into())
    }

}