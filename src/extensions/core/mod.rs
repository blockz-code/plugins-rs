mod fix;
mod dirs;

use crate::Output;
use deno_core::{ extension, op2 };

#[op2]
#[serde]
fn js_which(
    #[string] path: String
) -> std::io::Result<Output> {
    let w = fix::rust_which(path).unwrap();
    Ok(Output {
        data: format!("{}", w.display()).into(),
    })
}


#[op2]
#[serde]
fn nid(
    #[smi] size: usize
) -> std::io::Result<Output> {
    Ok(Output {
        data: nanoid::nanoid!(size).into(),
    })
}


#[op2]
#[serde]
fn nid_custom(
    #[smi] size: usize,
    #[string] custom: String
) -> std::io::Result<Output> {
    let mut v = vec![];
    custom.chars().for_each(|c| v.push(c));
    Ok(Output {
        data: nanoid::nanoid!(size, &v).into(),
    })
}


#[op2]
#[serde]
fn nid_safe(
    #[smi] size: usize,
) -> std::io::Result<Output> {
    Ok(Output {
        data: nanoid::nanoid!(size, &nanoid::alphabet::SAFE, random).into(),
    })
}


#[op2]
#[serde]
fn uid() -> std::io::Result<Output> {
    let uuid = uuid::Uuid::new_v4();
    Ok(Output { data: uuid.to_string() })
}







#[derive(serde::Deserialize, serde::Serialize)]
struct ConfigToml {
    name: String,
    version: String,
    homepage: String,
    repository: String,
}

#[op2]
#[serde]
fn config() -> ConfigToml {
    ConfigToml {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        homepage: env!("CARGO_PKG_HOMEPAGE").to_string(),
        repository: env!("CARGO_PKG_REPOSITORY").to_string(),
    }
}








//#[allow(clippy::match_single_binding)] // needed for temporary lifetime
#[op2(fast)]
fn op_internal_log(
    #[string] url: String,
    #[smi] level: u32,
    #[string] message: String,
) {
    let level = match level {
        1 => log::Level::Error.as_str(),
        2 => log::Level::Warn.as_str(),
        3 => log::Level::Info.as_str(),
        4 => log::Level::Debug.as_str(),
        5 => log::Level::Trace.as_str(),
        _ => unreachable!(),
    };
    println!("[{}] {} {}", level, &url, message);
}






extension!(
    core_js,
    ops = [
        uid,
        nid,
        nid_custom,
        nid_safe,
        js_which,
        config,

        op_internal_log,

        dirs::audio_dir,
        dirs::cache_dir,
        dirs::config_dir,
        dirs::config_local_dir,
        dirs::data_dir,
        dirs::data_local_dir,
        dirs::desktop_dir,
        dirs::document_dir,
        dirs::download_dir,
        dirs::home_dir,
        dirs::picture_dir,
        dirs::video_dir,
    ],
    esm_entry_point = "plugins-rs:core", 
    esm = [
        dir "src/extensions/core",
        "plugins-rs:core" = "index.js"
    ],
    docs = "Rust Based HTML Scraper", "scraper html from rust"
);

pub fn init() -> deno_core::Extension {
    core_js::init()
}




















//////////////////////
/// 
/// 

fn random (size: usize) -> Vec<u8> {
    let result: Vec<u8> = vec![0; size];
    result
}