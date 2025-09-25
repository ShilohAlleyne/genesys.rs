use std::{fmt, path::PathBuf};

#[derive(Debug)]
pub enum Error {
    Deserialization { path: PathBuf, source: serde_json::Error },
    Api(String),
    MissingFile(PathBuf),
    FileRead { path: PathBuf, source: std::io::Error },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Deserialization { path, source } => {
                write!(f, "Failed to deserialize {}: {}", path.display(), source)
            }
            Error::Api(msg) => write!(f, "API error: {}", msg),
            Error::MissingFile(path) => write!(f, "Missing file: {}", path.display()),
            Error::FileRead { path, source } => {
                write!(f, "Failed to read {}: {}", path.display(), source)
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::Api(value.to_string())
    }
}
