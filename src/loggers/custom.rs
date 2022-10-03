use crate::Config;
use crate::Log;

impl Config {
    /// logger `custom`
    /// the type of log is determined by the user of this library
    pub fn custom(&self, log_type: String, content: String) {
        self.write(Log::Custom(log_type, content));
    }
}
