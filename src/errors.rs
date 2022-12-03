use std::io;

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum CustomError {
    #[error("dzl IO Error: {0:?}")]
    IOError(String),
    #[error("dzl Parse Error {0:?}")]
    ParseError(String),
}

impl From<io::Error> for CustomError {
    fn from(e: io::Error) -> Self {
        match e.kind() {
            io::ErrorKind::NotFound => Self::IOError(String::from("File not found")),
            io::ErrorKind::PermissionDenied => Self::IOError(String::from(
                "Permission denied, please check the config file's permissions",
            )),
            _ => Self::IOError(format!("Other IO errors: {:?}", e.kind())),
        }
    }
}
