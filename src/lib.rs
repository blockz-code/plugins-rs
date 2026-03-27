mod extensions;
mod errors;
mod utils;
mod modules;


mod efs;
pub use efs::{EFsData, EFsPath, FsFile, PluginsFs};


pub use internal_macros::bind_dir;





pub use errors::{ Error, Result };


#[cfg(archives)]
pub use modules::ArchiveType;

pub use modules::{ Source, SourceType, Plugin };


use modules::{ ModuleLoader, SourceLoader };






use std::rc::Rc;
use std::sync::Arc;

use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use deno_core::PollEventLoopOptions;

use deno_core::{ serde_v8, v8 };





/*

    static:

*/


pub struct PluginSystem {
    sources : Arc<SourceLoader>,
    runtime : Option<JsRuntime>,
}

impl PluginSystem {

    //

    pub fn builder() -> Self {
        Self {
            runtime : None,
            sources : Arc::new(SourceLoader::new()),
        }
    }

    //

    pub fn add_embed(self, source: Source) -> Self {
        self.sources.add_embed(source);
        self
    }

    pub fn add_source(self, source: Source) -> Self {
        self.sources.add(source);
        self
    }

    //

    async fn set_runtime(&mut self) {

        let exstensions = vec![
            //deno_webidl::deno_webidl::init(),
            //deno_web::deno_web::init(Arc::new(Default::default()), None, Default::default()),

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
            extensions: exstensions,
            module_loader: Some(Rc::new(ModuleLoader::new(self.sources.clone()))),
            ..Default::default()
        }));

    }

    //

    async fn initialize(&mut self) -> Result<()> {

        self.set_runtime().await;

        let runtime = self.runtime.as_mut().unwrap();

        // Preload Entrys
        self.sources.preload()?;
        self.sources.plugins()?;

        // PluginSystem API
        runtime.execute_script("__RUNTIME_API__", include_str!("../api/index.js"))?;

        // Init Loaded Static/Embed
        self.sources.init_loaded(runtime).await?;

        // Run runtime Loop
        runtime.run_event_loop(PollEventLoopOptions::default()).await?;
        
        Ok(())
    }

    //

    /// use your custom rt to run.
    pub async fn run_into(mut self) -> crate::Result<PluginSystem> {
        self.initialize().await?;
        Ok(self)
    }

    /// run custom
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
        let code = format!(r#"globalThis.Plugins.loadPlugin("{}").{}"#, plugin, key);
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