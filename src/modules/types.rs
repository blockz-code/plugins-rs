use crate::efs::PluginsFs;
use std::path::PathBuf;
use deno_core::url::Url;

pub enum SourceEntry {
    Static(PathBuf),
    Embed(PluginsFs),
}

#[derive(Debug)]
pub struct SourceItem {
    /// Source ID (from vec)
    pub source: usize,
    /// Source scheme (embed/static/jsr/npm/git).
    pub scheme : String,
    /// Source Folder where plugin.json is.
    pub entry : String,
    /// File to Lookup
    pub search: String,
    /// Archive
    pub archive: Option<String>,
    /// Source File Data
    pub content: Option<Vec<u8>>,
}

#[allow(unused)]
#[derive(Debug)]
pub struct EmbedEntry {
    /// Source ID (from vec)
    pub source: usize,
    /// Source scheme (embed/static/jsr/npm/git).
    pub scheme : String,
    /// Source Folder where plugin.json is.
    pub entry : String,
    /// File to Lookup
    pub search: String,
    /// entry url will be set by the plugin system
    pub url: Option<Url>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Plugin {
    /// name
    pub name: String,
    /// description
    pub description: String,
    /// identifier like `com.example.plugin_name`
    pub identifier: String,
    /// version
    pub version: String,
    /// plugin entry like `index.ts`
    pub entry: String,

    #[serde(skip)]
    /// entry url will be set by the plugin system
    pub url: Option<Url>,
    #[serde(skip)]
    /// source id will be set by the plugin system
    pub source: Option<usize>,
    #[serde(skip)]
    /// base path where of plugin location / or archive
    pub base: Option<String>,
}