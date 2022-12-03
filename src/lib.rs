//! # dzl
//! A crate for logging.
//!
//! It is simple and easy to use
//!
//! You can learn more [here](https://github.com/za-songguo/dzl "Github")
//!
//! # Example
//! `main.rs`
//! ```
//! dzl::init().ok(); // Call this function only once in main.rs
//! dzl::loggers::trace("Something...");
//! dzl::loggers::debug("Something...");
//! dzl::loggers::info("Something...");
//! dzl::loggers::warn("Something...");
//! dzl::loggers::error("Something...");
//! dzl::loggers::custom("CustomType", "Something...");
//! ```
//!
//! `Dzl.toml`
//! ```toml
//! write_to_log_file = true
//! log_path = "dzl.log" # This file needs to be created
//! log_level = "debug" # Only logs greater than or equal to this level will be printed and written to the log file
//! ```
//! Output:
//! ```
//! 2022-12-03 11:30:55.23172315 +08:00:00 DEBUG Something...
//! 2022-12-03 11:30:55.233852405 +08:00:00 WARN Something...
//! 2022-12-03 11:30:55.235884013 +08:00:00 ERROR Something...
//! 2022-12-03 11:30:55.240158709 +08:00:00 CustomType Something...
//! ```
//!

use std::{fs, io::ErrorKind};

pub(crate) mod config;
pub(crate) mod errors;
pub(crate) mod log;
pub mod loggers;

/// Init function
pub fn init() -> Result<(), errors::CustomError> {
    let config = config::Config::new().read()?;
    if let Some(log_path) = config.log_path() {
        let file = fs::File::open(&log_path);
        if file.is_err() && file.unwrap_err().kind() == ErrorKind::NotFound {
            // Create the log file if the file is not exists
            fs::File::create(&log_path)?;
        }
    }

    Ok(())
}
