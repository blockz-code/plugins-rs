use super::op2;

#[op2]
#[serde]
pub fn audio_dir() -> std::io::Result<super::Output> {
    Ok(super::Output { data: dirs::audio_dir().unwrap().as_path().to_string_lossy().to_string() })
}

#[op2]
#[serde]
pub fn cache_dir() -> std::io::Result<super::Output> {
    Ok(super::Output { data: dirs::cache_dir().unwrap().as_path().to_string_lossy().to_string() })
}

#[op2]
#[serde]
pub fn config_dir() -> std::io::Result<super::Output> {
    Ok(super::Output { data: dirs::config_dir().unwrap().as_path().to_string_lossy().to_string() })
}

#[op2]
#[serde]
pub fn config_local_dir() -> std::io::Result<super::Output> {
    Ok(super::Output { data: dirs::config_local_dir().unwrap().as_path().to_string_lossy().to_string() })
}

#[op2]
#[serde]
pub fn data_dir() -> std::io::Result<super::Output> {
    Ok(super::Output { data: dirs::data_dir().unwrap().as_path().to_string_lossy().to_string() })
}

#[op2]
#[serde]
pub fn data_local_dir() -> std::io::Result<super::Output> {
    Ok(super::Output { data: dirs::data_local_dir().unwrap().as_path().to_string_lossy().to_string() })
}

#[op2]
#[serde]
pub fn desktop_dir() -> std::io::Result<super::Output> {
    Ok(super::Output { data: dirs::desktop_dir().unwrap().as_path().to_string_lossy().to_string() })
}

#[op2]
#[serde]
pub fn document_dir() -> std::io::Result<super::Output> {
    Ok(super::Output { data: dirs::document_dir().unwrap().as_path().to_string_lossy().to_string() })
}

#[op2]
#[serde]
pub fn download_dir() -> std::io::Result<super::Output> {
    Ok(super::Output { data: dirs::download_dir().unwrap().as_path().to_string_lossy().to_string() })
}

#[op2]
#[serde]
pub fn home_dir() -> std::io::Result<super::Output> {
    Ok(super::Output { data: dirs::home_dir().unwrap().as_path().to_string_lossy().to_string() })
}

#[op2]
#[serde]
pub fn picture_dir() -> std::io::Result<super::Output> {
    Ok(super::Output { data: dirs::picture_dir().unwrap().as_path().to_string_lossy().to_string() })
}

#[op2]
#[serde]
pub fn video_dir() -> std::io::Result<super::Output> {
    Ok(super::Output { data: dirs::video_dir().unwrap().as_path().to_string_lossy().to_string() })
}