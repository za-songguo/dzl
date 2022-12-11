use std::io;

#[derive(Debug, Clone)]
pub enum CustomError {
    IOError(String),
    ParseError(String),
}

impl From<io::Error> for CustomError {
    fn from(e: io::Error) -> Self {
        match e.kind() {
            io::ErrorKind::NotFound => {
                Self::IOError(String::from("Config file `Dzl.toml` not found"))
            }
            io::ErrorKind::PermissionDenied => Self::IOError(String::from(
                "Permission denied, please check the config file's permissions",
            )),
            _ => Self::IOError(format!("Other IO errors: {:?}", e.kind())),
        }
    }
}
