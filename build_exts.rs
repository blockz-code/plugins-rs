use deno_core::extension;

extension!(
    core_js, 
    esm_entry_point = "plugins-rs:core", 
    esm = [
        dir "src/extensions/core",
        "plugins-rs:core" = "index.js"
    ]
);

#[cfg(feature = "media")]
extension!(
    media_js, 
    esm_entry_point = "plugins-rs:media", 
    esm = [
        dir "src/extensions/media",
        "plugins-rs:media" = "index.js"
    ]
);

#[cfg(feature = "scrape")]
extension!(
    scrape_js, 
    esm_entry_point = "plugins-rs:scrape", 
    esm = [
        dir "src/extensions/scrape",
        "plugins-rs:scrape" = "index.js"
    ]
);