use crate::Config;
use crate::Log;

impl Config {
    /// logger `warn`
    pub fn warn(&self, content: String) {
        self.write(Log::Warn(content));
    }
}
