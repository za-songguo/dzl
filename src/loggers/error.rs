use crate::Config;
use crate::Log;

impl Config {
    /// logger `error`
    pub fn error(&self, content: String) {
        self.write(Log::Error(content));
    }
}
