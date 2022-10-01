/// logger `warn`
use crate::Config;
use crate::Log;

impl Config {
    pub fn warn(&self, content: String) {
        self.write(Log::Warn(content));
    }
}
