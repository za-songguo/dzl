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
//! // let dzl_config = Config::new(false,true, "dzl.log".to_string());
//!
//! // without color and emojis
//! // let dzl_config = Config::new(true, false, "dzl. log".to_string());
//!
//! // use default config
//! let dzl_config = Config::default_config();
//! // initializing
//! dzl::init(&dzl_config);
//!
//! // logging, you can also use `format!()` macro to format the string
//! dzl_config.trace("dzl".to_string());
//! dzl_config.info("dzl".to_string());
//! dzl_config.debug("debug".to_string());
//! dzl_config.warn("dzl".to_string());
//! dzl_config.error("dzl".to_string());
//! // custom log type
//! dzl_config.custom("CustomType".to_string(), "dzl".to_string());
//! ```
use std::fs::{self, read_to_string, File};
use std::io::{Read, Write};
use std::path::Path;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
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
        content.push_str(&config.prepare_log(Log::Init(String::from("Called init() function"))));

        // write to the file
        if let Err(err) = fs::write(path, content) {
            config.self_error(format! ("Cannot write `init` message to the log file. Error: {:#}. The content of log will not write to the log file.", err));
        }
    } else {
        let _ = &config.prepare_log(Log::Init(
            "Because of the configuration, logs will not write to the log file".to_string(),
        ));
    }
}

/// the configuration of the loggers
#[derive(Debug)]
pub struct Config {
    pub write_logs_to_files: bool,
    pub with_color_and_emojis: bool,
    pub log_path: String,
}

impl Config {
    /// create a new logger configuration
    pub fn new(write_logs_to_files: bool, with_color_and_emojis: bool, log_path: String) -> Self {
        Self {
            write_logs_to_files,
            with_color_and_emojis,
            log_path,
        }
    }

    /// the default configuration of struct `Config`
    pub fn default_config() -> Self {
        Self {
            write_logs_to_files: true,
            with_color_and_emojis: true,
            log_path: "dzl.log".to_string(),
        }
    }

    // write something to the log file
    fn write(&self, content: Log) {
        if self.write_logs_to_files {
            // print and record logs with color and emojis
            let path = Path::new(self.log_path.as_str());
            // log file should exist after calling the `init` function
            // read file
            if let Ok(mut file_content) = read_to_string(path) {
                // push
                file_content.push_str(self.prepare_log(content).as_str());
                // write to the log file
                if let Err(err) = fs::write(path, file_content) {
                    self.self_error(format!("Failed to write log to the log file! Error: {err}"));
                }
            } else {
                self.self_error("Cannot read log file to string. It may be that the file does not exist or does not have read permissions, or the file is not the valid UTF-8 format. You may not have called init() function. You should call it before calling logging functions!!!".to_string());
            }
        } else {
            // only print the content of the log
            self.prepare_log(content);
        }
    }

    fn prepare_log(&self, log: Log) -> String {
        // prepare the log with the default template
        if self.with_color_and_emojis {
            // with color and emojis
            match log {
                Log::Error(content) => write_color(
                    true,
                    Color::Red,
                    format!(
                        "[dzl 😕] Log Type: Error, Time: {}, message: {}\n\n",
                        OffsetDateTime::now_local().unwrap(),
                        content
                    ),
                ),
                Log::Trace(content) => write_color(
                    false,
                    Color::White,
                    format!(
                        "[dzl 💬] Log Type: Trace, Time: {}, message: {}\n\n",
                        OffsetDateTime::now_local().unwrap(),
                        content
                    ),
                ),
                Log::Info(content) => write_color(
                    false,
                    Color::Green,
                    format!(
                        "[dzl 💬] Log Type: Info, Time: {}, message: {}\n\n",
                        OffsetDateTime::now_local().unwrap(),
                        content
                    ),
                ),
                Log::Debug(content) => write_color(
                    false,
                    Color::Green,
                    format!(
                        "[dzl 💬] Log Type: Debug, Time: {}, message: {}\n\n",
                        OffsetDateTime::now_local().unwrap(),
                        content
                    ),
                ),
                Log::Warn(content) => write_color(
                    false,
                    Color::Yellow,
                    format!(
                        "[dzl 😐] Log Type: Warn, Time: {}, message: {}\n\n",
                        OffsetDateTime::now_local().unwrap(),
                        content
                    ),
                ),

                Log::Init(content) => write_color(
                    false,
                    Color::Blue,
                    format!(
                        "[dzl 😀] Log Type: Init, Time: {}, message: {}\n\n",
                        OffsetDateTime::now_local().unwrap(),
                        content
                    ),
                ),
                Log::Custom(log_type, content) => write_color(
                    false,
                    Color::Cyan,
                    format!(
                        "[dzl 🌱] Log Type: {}, Time: {}, message: {}\n\n",
                        log_type,
                        OffsetDateTime::now_local().unwrap(),
                        content
                    ),
                ),
            }
        } else {
            // without color and emojis
            // and print log to stdout or stderr
            Self::match_and_print(log)
        }
    }

    // match the log type and print
    // (without emojis and color)
    fn match_and_print(log: Log) -> String {
        match log {
            Log::Error(content) => {
                eprintln!(
                    "[dzl] Log Type: Error, Time: {}, message: {}",
                    OffsetDateTime::now_local().unwrap(),
                    content
                );
                format!(
                    "[dzl] Log Type: Error, Time: {}, message: {}\n\n",
                    OffsetDateTime::now_local().unwrap(),
                    content
                )
            }
            Log::Trace(content) => {
                println!(
                    "[dzl] Log Type: Trace, Time: {}, message: {}",
                    OffsetDateTime::now_local().unwrap(),
                    content
                );
                format!(
                    "[dzl] Log Type: Trace, Time: {}, message: {}\n\n",
                    OffsetDateTime::now_local().unwrap(),
                    content
                )
            }
            Log::Info(content) => {
                println!(
                    "[dzl] Log Type: Info, Time: {}, message: {}",
                    OffsetDateTime::now_local().unwrap(),
                    content
                );
                format!(
                    "[dzl] Log Type: Info, Time: {}, message: {}\n\n",
                    OffsetDateTime::now_local().unwrap(),
                    content
                )
            }
            Log::Init(content) => {
                println!(
                    "[dzl] Log Type: Init, Time: {}, message: {}",
                    OffsetDateTime::now_local().unwrap(),
                    content
                );
                format!(
                    "[dzl] Log Type: Init, Time: {}, message: {}\n\n",
                    OffsetDateTime::now_local().unwrap(),
                    content
                )
            }
            Log::Debug(content) => {
                println!(
                    "[dzl] Log Type: Debug, Time: {}, message: {}",
                    OffsetDateTime::now_local().unwrap(),
                    content
                );
                format!(
                    "[dzl] Log Type: Debug, Time: {}, message: {}\n\n",
                    OffsetDateTime::now_local().unwrap(),
                    content
                )
            }
            Log::Warn(content) => {
                println!(
                    "[dzl] Log Type: Warn, Time: {}, message: {}",
                    OffsetDateTime::now_local().unwrap(),
                    content
                );
                format!(
                    "[dzl] Log Type: Warn, Time: {}, message: {}\n\n",
                    OffsetDateTime::now_local().unwrap(),
                    content
                )
            }
            Log::Custom(log_type, content) => {
                println!(
                    "[dzl] Log Type: {}, Time: {}, message: {}",
                    log_type,
                    OffsetDateTime::now_local().unwrap(),
                    content
                );
                format!(
                    "[dzl] Log Type: {}, Time: {}, message: {}\n\n",
                    log_type,
                    OffsetDateTime::now_local().unwrap(),
                    content
                )
            }
        }
    }
}

// write something to console with color
fn write_color<T: std::fmt::Display>(is_err: bool, color: Color, content: T) -> String {
    // stderr or stdout
    if is_err {
        let mut stderr = StandardStream::stderr(ColorChoice::Always);
        stderr
            .set_color(ColorSpec::new().set_fg(Some(color)))
            .unwrap_or_else(|e| {
                eprintln!("[dzl] Failed to write messages with color in console! Error: {e}");
            });
        writeln!(&mut stderr, "{}", content.to_string().trim()).unwrap();
        content.to_string()
    } else {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout
            .set_color(ColorSpec::new().set_fg(Some(color)))
            .unwrap_or_else(|e| {
                eprintln!("[dzl] Failed to write messages with color in console! Error: {e}");
            });
        writeln!(&mut stdout, "{}", content.to_string().trim()).unwrap();
        content.to_string()
    }
}

// types of logs
enum Log {
    Init(String),
    Trace(String),
    Info(String),
    Debug(String),
    Warn(String),
    Error(String),
    /// (LogType, Content)
    Custom(String, String),
}
