use crate::{errors::CustomError, log};

pub fn custom<T: std::fmt::Display>(log_type: T, content: T) {
    handle_error(log::log(log::Log::Custom(
        log_type.to_string(),
        content.to_string(),
    )));
}
pub fn trace<T: std::fmt::Display>(content: T) {
    handle_error(log::log(log::Log::Trace(content.to_string())));
}

pub fn debug<T: std::fmt::Display>(content: T) {
    handle_error(log::log(log::Log::Debug(content.to_string())));
}

pub fn info<T: std::fmt::Display>(content: T) {
    handle_error(log::log(log::Log::Info(content.to_string())));
}

pub fn warn<T: std::fmt::Display>(content: T) {
    handle_error(log::log(log::Log::Warn(content.to_string())));
}

pub fn error<T: std::fmt::Display>(content: T) {
    handle_error(log::log(log::Log::Error(content.to_string())));
}

/// Print the error message if the parameter is an `Err` value
fn handle_error(result: Result<(), CustomError>) {
    if let Err(e) = result {
        match e {
            CustomError::IOError(msg) => eprintln!("{}", msg),
            CustomError::ParseError(msg) => eprintln!("{}", msg),
        }
        std::process::exit(1)
    }
}
