use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;


use deno_core::ModuleLoadOptions;
use deno_core::ModuleLoadReferrer;
use deno_core::ModuleLoadResponse;
use deno_core::ModuleLoader;
use deno_core::ModuleSource;
use deno_core::ModuleSourceCode;
use deno_core::ModuleSpecifier;
use deno_core::ResolutionKind;
use deno_core::error::ModuleLoaderError;
use deno_core::resolve_import;
use deno_error::JsErrorBox;


use crate::{ Reader, SourceMapStore, transpile, mediatype, url_info };

pub struct Loader {
    reader: Reader,
    source_maps: SourceMapStore,
}



impl Loader {
    pub fn new(reader: Reader) -> Self {
        Self {
            reader,
            source_maps: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}



impl ModuleLoader for Loader {

    fn resolve(&self, specifier: &str, referrer: &str, _kind: ResolutionKind) -> std::result::Result<ModuleSpecifier, ModuleLoaderError> {
        resolve_import(specifier, referrer).map_err(JsErrorBox::from_err)
    }

    fn load(&self, module_specifier: &ModuleSpecifier, _maybe_referrer: Option<&ModuleLoadReferrer>, _options: ModuleLoadOptions) -> ModuleLoadResponse {

        let source_maps = self.source_maps.clone();

        fn load(
            reader: Reader,
            source_maps: SourceMapStore,
            module_specifier: &ModuleSpecifier,
        ) -> std::result::Result<ModuleSource, ModuleLoaderError> {

            match module_specifier.scheme() {
                "static" | "embed" => {

                    let url = url_info(module_specifier.clone()).map_err(|e| JsErrorBox::generic(e.to_string()))?;
                    let (media_type, module_type, should_transpile) = mediatype(url.search.to_str().unwrap()).map_err(|e| JsErrorBox::generic(e.to_string()))?;
                    let data = reader.find(url.source, url.entry, url.search).map_err(|e| JsErrorBox::generic(e.to_string()))?;
                    let code = transpile(String::from_utf8_lossy(&data).to_string(), source_maps, module_specifier, media_type, should_transpile).map_err(|e| JsErrorBox::generic(e.to_string()))?;
                   
                    Ok(ModuleSource::new(module_type, ModuleSourceCode::String(code.into()), module_specifier, None))

                }
                //"jsr" => {}
                scheme => unimplemented!("{}", format!("scheme [{}] not implemented.", scheme))
            }

        }

        ModuleLoadResponse::Sync(load(self.reader.clone(), source_maps, module_specifier))

    }

    fn get_source_map(&self, specifier: &str) -> Option<Cow<'_, [u8]>> {
        self
            .source_maps
            .borrow()
            .get(specifier)
            .map(|v| v.clone().into())
    }

}