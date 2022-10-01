// logger `self_error` for crate's error
// it can only use in this crate
use crate::Config;
use time::OffsetDateTime;

impl Config {
    pub fn self_error(&self, content: String) {
        // only print the content to stderr because `self_error` usually is file system error so we cannot write log to the log file
        eprintln!(
            "[dzl] Error Type: SelfError, Time: {}, message: {:#}",
            OffsetDateTime::now_local().unwrap(),
            content
        )
    }
}
