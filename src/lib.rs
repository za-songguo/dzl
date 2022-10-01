//! # dzl
//! This is a library crate for logging.
//!
//! It is simple and easy to use
//! and it is lightweight and has a few dependencies :)
//!
//! You can learn more [here](https://github.com/za-songguo/dzl "Github")
//!
//! # Example
//! ```
//! use dzl::Config;
//!
//! // not write logs to files
//! // let dzl_config = Config::new(false, "dzl.log".to_string());
//!
//! // use default config
//! let dzl_config = Config::default_config();
//! // initializing
//! dzl::init(&dzl_config);
//!
//! // logging, you can also use `format!()` macro to format the string
//! dzl_config.trace("dzl".to_string());
//! dzl_config.info("dzl".to_string());
//! dzl_config.warn("dzl".to_string());
//! dzl_config.error("dzl".to_string());
//! // custom log type
//! dzl_config.custom("CustomType".to_string(), "dzl".to_string());
//! ```
use std::fs::{self, read_to_string, File};
use std::io::Read;
use std::path::Path;
use time::OffsetDateTime;

pub mod loggers;

/// use this function to initialize the log system
/// the function requires an argument of type `&Config`:  `config`
pub fn init(config: &Config) {
    if config.write_logs_to_files {
        // create a log file if it is not exist
        let path = Path::new(config.log_path.as_str());
        // if the path does not exist
        if !path.exists() {
            // create the log file
            if let Err(err) = File::create(&path) {
                config.self_error(format!("Cannot create the log file. Error: {:#}. The content of log will not write to the log file.", err));
            }
        }

        // if the file exists or after creating the log file
        // write `init` message in the log file
        let mut content = String::new();

        // read the log file to string => content
        match File::open(&path) {
            Ok(mut file) => {
                // reading to string error
                if let Err(err) = file.read_to_string(&mut content) {
                    config.self_error(format!("Cannot read the log file. Error: {:#}. The content of log will not write to the log file.", err));
                }
            }
            // opening file error
            Err(err) => {
                config.self_error(format!("Cannot open the log file. Error: {:#}. The content of log will not write to the log file.", err));
            }
        }

        // PUSH message
        content.push_str(&prepare_log(Log::Init(String::from(
            "Called init() function",
        ))));

        // write to the file
        if let Err(err) = fs::write(path, content) {
            config.self_error(format! ("Cannot write `init` message to the log file. Error: {:#}. The content of log will not write to the log file.", err));
        }
    } else {
        prepare_log(Log::Init(
            "Because of the configuration, logs will not write to the log file".to_string(),
        ));
    }
}

#[derive(Debug)]
pub struct Config {
    pub write_logs_to_files: bool,
    pub log_path: String,
}

impl Config {
    /// create a new logger configuration
    pub fn new(write_logs_to_files: bool, log_path: String) -> Self {
        Self {
            write_logs_to_files,
            log_path,
        }
    }

    /// the default configuration of struct `Config`
    pub fn default_config() -> Self {
        Self {
            write_logs_to_files: true,
            log_path: "dzl.log".to_string(),
        }
    }

    // write something to the log file
    fn write(&self, content: Log) {
        if self.write_logs_to_files {
            let path = Path::new(self.log_path.as_str());
            // log file should exist after calling the `init` function
            // read file
            if let Ok(mut file_content) = read_to_string(path) {
                // push
                file_content.push_str(prepare_log(content).as_str());
                // write to the log file
                if let Err(err) = fs::write(path, file_content) {
                    self.self_error(format!("Failed to write log to the log file! Error: {err}"));
                }
            } else {
                self.self_error("Cannot read log file to string. It may be that the file does not exist or does not have read permissions, or the file is not the valid UTF-8 format. You may not have called init() function. You should call it before calling logging functions!!!".to_string());
            }
        } else {
            // only print the content of the log
            prepare_log(content);
        }
    }
}

fn prepare_log(log: Log) -> String {
    // prepare the log with the default template
    let (log_type, content) = log.log_type();

    // the `content` in the match expression is the value inside the variants of `Log`
    // print log to stdout or stderr
    match log_type.as_str() {
        "Error" => eprintln!(
            "[dzl] Log Type: {}, Time: {}, message: {}",
            log_type,
            OffsetDateTime::now_local().unwrap(),
            content
        ),
        _ => println!(
            "[dzl] Log Type: {}, Time: {}, message: {}",
            log_type,
            OffsetDateTime::now_local().unwrap(),
            content
        ),
    }
    // mutable `content`
    let mut content = content;

    // use `=` change all the content of the var `content`
    content = format!(
        "[dzl] Log Type: {}, Time: {}, message: {}\n\n",
        log_type,
        OffsetDateTime::now_local().unwrap(),
        content
    );
    content
}

// types of logs
enum Log {
    Init(String),
    Trace(String),
    Info(String),
    Warn(String),
    Error(String),
    /// (LogType, Content)
    Custom(String, String),
}

impl Log {
    // return (LogType, Content)
    fn log_type(&self) -> (String, String) {
        match self {
            Self::Init(content) => ("Init".to_string(), content.clone()),
            Self::Trace(content) => ("Trace".to_string(), content.clone()),
            Self::Info(content) => ("Info".to_string(), content.clone()),
            Self::Warn(content) => ("Warn".to_string(), content.clone()),
            Self::Error(content) => ("Error".to_string(), content.clone()),
            Self::Custom(log_type, content) => (log_type.clone(), content.clone()),
        }
    }
}
