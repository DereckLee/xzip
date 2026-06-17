use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RzipError {
    #[error("unsupported encoding '{0}'. try one of: utf-8, gbk, cp936, shift_jis")]
    UnsupportedEncoding(String),
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error("zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("cannot encode path '{path}' using {encoding}")]
    EncodePath {
        path: String,
        encoding: &'static str,
    },
    #[error("cannot decode zip entry name using {encoding}")]
    DecodeEntryName { encoding: &'static str },
    #[error("invalid glob pattern: {0}")]
    InvalidGlobPattern(String),
    #[error("unsafe zip entry path escapes output directory: {0}")]
    UnsafePath(String),
    #[error("invalid input path: {0}")]
    InvalidInput(PathBuf),
}
