/// logger `error`
use crate::Config;
use crate::Log;

impl Config {
    pub fn error(&self, content: String) {
        self.write(Log::Error(content));
    }
}
