pub type Result<T> = std::result::Result<T, Error>;

//
// Errors
//

#[derive(Debug, thiserror::Error)]
pub enum Error {

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    StripPrefixError(#[from] std::path::StripPrefixError),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    DenoCoreError(#[from] deno_core::error::CoreError),

    #[error(transparent)]
    DenoCoreModuleResolutionError(#[from] deno_core::ModuleResolutionError),

    #[error(transparent)]
    DenoCoreAnyhowError(#[from] deno_core::anyhow::Error),

    #[error(transparent)]
    WhichError(#[from] which::Error),

    #[error(transparent)]
    DenoCoreJsError(#[from] deno_core::error::JsError),

    #[error(transparent)]
    DenoCoreJsError2(#[from] Box<deno_core::error::JsError>),

    #[error(transparent)]
    DenoCoreSerdeV8Error(#[from] deno_core::serde_v8::Error),

    #[error(transparent)]
    DenoCoreJsErrorBox(#[from] deno_error::JsErrorBox),

    #[error(transparent)]
    DenoCoreParseDiagnosticError(#[from] deno_ast::ParseDiagnostic),

    #[error(transparent)]
    DenoCoreTranspileError(#[from] deno_ast::TranspileError),


    
    #[cfg(feature = "zip")]
    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),

    #[cfg(feature = "7z")]
    #[error(transparent)]
    SevenzRust2Error(#[from] sevenz_rust2::Error),

    #[cfg(any(feature = "media"))]
    #[error(transparent)]
    FFbinsError(#[from] ffbins_rs::Error),

    #[cfg(any(feature = "media", feature = "scrape"))]
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),


    


    #[error("URL parse error")]
    UrlParse(#[from] deno_core::url::ParseError),

    #[error("unknown error: {0}")]
    Unknown(String),

    #[error("Parse error")]
    Parse(#[from] std::num::ParseIntError),

}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}