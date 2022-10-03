use crate::Config;
use crate::Log;

impl Config {
    /// logger `info`
    pub fn info(&self, content: String) {
        self.write(Log::Info(content));
    }
}
