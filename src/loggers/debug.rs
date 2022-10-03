use crate::Config;
use crate::Log;

impl Config {
    /// logger `debug`
    pub fn debug(&self, content: String) {
        self.write(Log::Debug(content));
    }
}
