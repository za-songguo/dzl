use std::fs;

use serde::Deserialize;

use crate::errors::CustomError;
use crate::log::Log;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    write_to_log_file: Option<bool>,
    log_path: Option<String>,
    log_level: Option<String>,
}

impl Config {
    // New Config (With default value)
    pub fn new() -> Self {
        Self::default()
    }

    /// Read config from the config file
    pub fn read(&self) -> Result<Self, CustomError> {
        let file = fs::read_to_string("Dzl.toml")?;
        let config = toml::from_str::<Config>(&file);
        if config.is_err() {
            return Err(CustomError::ParseError(String::from(
                "Failed to parse the config file, please check your file format",
            )));
        }
        let config = config.unwrap();
        Ok(Self {
            write_to_log_file: config.write_to_log_file(),
            log_path: config.log_path(),
            log_level: config.log_level,
        })
    }

    // Getters

    pub fn log_path(&self) -> Option<String> {
        self.log_path.clone()
    }
    pub fn log_level(&self) -> Result<Option<Log>, CustomError> {
        if let Some(level) = &self.log_level {
            match level.as_str() {
                "trace" => Ok(Some(Log::Trace("".into()))),
                "debug" => Ok(Some(Log::Debug("".into()))),
                "info" => Ok(Some(Log::Info("".into()))),
                "warn" => Ok(Some(Log::Warn("".into()))),
                "error" => Ok(Some(Log::Error("".into()))),
                "custom" => Ok(Some(Log::Custom("".into(), "".into()))),
                _ => Err(CustomError::ParseError("Unknown log level".into())),
            }
        } else {
            Ok(None)
        }
    }
    pub fn write_to_log_file(&self) -> Option<bool> {
        self.write_to_log_file
    }
}

impl Default for Config {
    /// The default config
    fn default() -> Self {
        Self {
            write_to_log_file: Some(true),
            log_path: Some("dzl.log".to_string()),
            log_level: Some("custom".into()),
        }
    }
}
