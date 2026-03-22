mod archives;
mod extensions;
mod errors;
mod loader;
#[allow(unused)]
mod loader_utils;
mod reader;
mod sources;
mod utils;

pub use errors::{ Error, Result };

pub use sources::{Source, Sources, SourceRes};

pub use loader::Loader;

pub use reader::{ Reader };

pub use utils::{ get_ext, get_type };
pub use utils::{ url_info, url_create };
pub use utils::{ File, FindType, SourceMapStore };
pub use utils::{ is_archive, is_file, is_supported };
pub use utils::{ create_file, transpile, mediatype };


pub use include_dir;



use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

use deno_core::url::Url;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use deno_core::PollEventLoopOptions;

use deno_core::{ serde_v8, v8 };




//static RUNTIME_SNAPSHOT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/RUNJS_SNAPSHOT.bin"));




pub struct PluginSystem {
    reader : Reader,
    runtime : Option<JsRuntime>,
    plugins : Arc<Mutex<Vec<Plugin>>>,
}

impl PluginSystem {

    //

    pub fn builder() -> Self {
        Self {
            runtime : None,
            reader : Reader::new(),
            plugins : Arc::new(Mutex::new(Vec::new())),
        }
    }

    //

    pub fn add_source(self, source: Source) -> Self {
        self.reader.sources.set(source);
        self
    }

    //

    async fn set_runtime(&mut self) {

        let exstensions = vec![
            crate::extensions::core::init(),
            #[cfg(feature = "media")]
            crate::extensions::media::init(),
            #[cfg(feature = "scrape")]
            crate::extensions::scrape::init(),
        ];

        self.runtime = Some(JsRuntime::new(RuntimeOptions {
            is_main: true,
            inspector: false,
            startup_snapshot: None,
            //startup_snapshot: Some(RUNTIME_SNAPSHOT),
            extensions: exstensions,
            module_loader: Some(Rc::new(Loader::new(self.reader.clone()))),
            ..Default::default()
        }));

    }

    //

    async fn load_sources(&mut self) -> Result<()> {

        self.set_runtime().await;
        
        self.reader.clone().load_plugins(|source_num,  result | {

            let mut plugins = self.plugins.lock().unwrap();

            let mut json: Plugin = serde_json::from_slice(&result.data.unwrap()).unwrap();
            let url = url_create(result.scheme, &json.identifier, json.entry.clone().into(), source_num, result.entry).unwrap();
            json.set_source(source_num);
            json.set_url(url);
            plugins.push(json);

            drop(plugins);

        })?;

        Ok(())
    }

    //

    async fn initialize(&mut self) -> Result<()> {

        self.load_sources().await?;

        let runtime = self.runtime.as_mut().unwrap();

        let mut plugins = self.plugins.lock().unwrap();

        runtime.execute_script("__RUNTIME_API__", include_str!("../api/index.js"))?;

        for item in plugins.iter_mut() {
            let mod_id = runtime.load_side_es_module(&item.url()).await?;
            let result = runtime.mod_evaluate(mod_id);
            result.await?;
        }

        runtime.run_event_loop(PollEventLoopOptions::default()).await?;

        drop(plugins);
        
        Ok(())
    }

    //

    pub fn run(mut self) -> crate::Result<PluginSystem> {
        let art = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        art.block_on(self.initialize())?;
        Ok(self)
    }

    //
    //
    //
    // PUBLIC FUNCTIONS
    //
    //
    //

    pub fn execute(&mut self, namespace: &'static str, plugin: &'static str, key: &'static str) -> crate::Result<serde_json::Value> {
        let code = format!(r#"(window.loadPlugin("{}").{})"#, plugin, key);
        let runtime = self.runtime.as_mut().unwrap();
        let result = runtime.execute_script(namespace, code)?;
        deno_core::scope!(scope, runtime);
        let local = v8::Local::new(scope, result);
        Ok(serde_v8::from_v8::<serde_json::Value>(scope, local)?)
    }

    //

    pub fn eval(&mut self, namespace: &'static str, message: &'static str) -> crate::Result<serde_json::Value> {
        let runtime = self.runtime.as_mut().unwrap();
        let result = runtime.execute_script(namespace, message)?;
        deno_core::scope!(scope, runtime);
        let local = v8::Local::new(scope, result);
        Ok(serde_v8::from_v8::<serde_json::Value>(scope, local)?)
    }
    
    //

    pub fn send(&mut self, namespace: &'static str, message: &'static str) -> crate::Result<String> {
        let runtime = self.runtime.as_mut().unwrap();
        let result = runtime.execute_script(namespace, message)?;
        deno_core::scope!(scope, runtime);
        let local = v8::Local::new(scope, result);
        Ok(serde_v8::from_v8::<serde_json::Value>(scope, local)?.to_string())
    }
    
    //

}









#[derive(serde::Serialize)]
pub struct Output {
    pub data: String,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Plugin {
    pub name: String,
    pub description: String,
    pub identifier: String,
    pub version: String,
    pub entry: String,

    #[serde(skip)]
    url: Option<Url>,
    #[serde(skip)]
    source: Option<usize>,
}

impl Plugin {
    
    pub fn set_url(&mut self, url: Url) {
        self.url = Some(url);
    }
    
    pub fn set_source(&mut self, source: usize) {
        self.source = Some(source);
    }

    #[allow(unused)]
    pub fn url(&self) -> Url {
        self.url.clone().unwrap()
    }

    #[allow(unused)]
    pub fn source(&self) -> usize {
        self.source.clone().unwrap()
    }

}