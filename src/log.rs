use crate::{config::Config, errors::CustomError};
use serde::Deserialize;
use std::fs;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum Log {
    Trace(String),
    Info(String),
    Debug(String),
    Warn(String),
    Error(String),
    /// (LogType, Content)
    Custom(String, String),
}

pub fn log(log: Log) -> Result<(), CustomError> {
    let config = Config::new().read()?;
    let ok_level = log.check_level(&config)?;
    if ok_level {
        let write_log_to_files = config.write_to_log_file().unwrap_or(false);
        if write_log_to_files {
            // Default: true
            print_log(&log)?;
            write_log(&config, &log)?;
        } else {
            print_log(&log)?;
        }
    }
    Ok(())
}

pub fn write_log(config: &Config, log: &Log) -> Result<(), CustomError> {
    let content = match log {
        Log::Trace(l) => l,
        Log::Info(l) => l,
        Log::Debug(l) => l,
        Log::Warn(l) => l,
        Log::Error(l) => l,
        Log::Custom(_, l) => l,
    };
    let new_log_content = format!(
        "{} {} {}\n",
        OffsetDateTime::now_local().unwrap(),
        log.get_level(),
        content
    );
    let log_file = fs::read_to_string(
        config
            .log_path()
            .expect("`log_path` must set with `write_log_to_files`"),
    );
    // Write to the log file
    // We checked it in`lib.rs`, check again here
    if let Ok(mut log_file) = log_file {
        log_file.push_str(&new_log_content);
        // Write the log to log file
        fs::write(
            config
                .log_path()
                .expect("`log_path` must set with `write_log_to_files`"),
            log_file,
        )?;
    } else {
        println!("Log file was not created, creating...");
        // Create the log file
        if let Some(log_path) = config.log_path() {
            if fs::File::open(&log_path).is_err() {
                // Create
                fs::File::create(&log_path)?;
                write_log(config, log).ok();
            }
        }
    }
    Ok(())
}

pub fn print_log(log: &Log) -> Result<(), CustomError> {
    let color = log.get_color();
    // Stderr or stdout (write_to_term())
    match log {
        Log::Error(l) => {
            let mut stderr = StandardStream::stderr(ColorChoice::Auto);
            stderr.set_color(ColorSpec::new().set_fg(Some(color)))?;
            write!(&mut stderr, "{} ", OffsetDateTime::now_local().unwrap()).unwrap();
            write!(&mut stderr, "{} ", log.get_level()).unwrap();
            writeln!(&mut stderr, "{}", l).unwrap();
        }
        Log::Trace(l) => write_to_term(log, l, None)?,
        Log::Info(l) => write_to_term(log, l, None)?,
        Log::Debug(l) => write_to_term(log, l, None)?,
        Log::Warn(l) => write_to_term(log, l, None)?,
        Log::Custom(t, l) => write_to_term(log, l, Some(t))?,
    }

    Ok(())
}

fn write_to_term(log: &Log, l: &String, custom_type: Option<&String>) -> Result<(), CustomError> {
    let color = log.get_color();
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
    write!(&mut stdout, "{} ", OffsetDateTime::now_local().unwrap()).unwrap();
    if let Some(t) = custom_type {
        write!(&mut stdout, "{} ", t).unwrap();
    } else {
        write!(&mut stdout, "{} ", log.get_level()).unwrap();
    }
    writeln!(&mut stdout, "{}", l).unwrap();

    Ok(())
}

impl Log {
    fn get_level(&self) -> &str {
        match self {
            Self::Custom(t, _) => t,
            Self::Trace(_) => "TRACE",
            Self::Info(_) => "INFO",
            Self::Debug(_) => "DEBUG",
            Self::Warn(_) => "WARN",
            Self::Error(_) => "ERROR",
        }
    }

    fn get_color(&self) -> Color {
        match self {
            Self::Trace(_) => Color::Green,
            Self::Info(_) => Color::Green,
            Self::Debug(_) => Color::Green,
            Self::Warn(_) => Color::Yellow,
            Self::Error(_) => Color::Red,
            Self::Custom(_, _) => Color::Blue,
        }
    }

    /// true => print || write, false => do nothing
    fn check_level(&self, config: &Config) -> Result<bool, CustomError> {
        let level = config.log_level()?;
        if let Some(level) = level {
            Ok(level <= *self)
        } else {
            Ok(true)
        }
    }
}
