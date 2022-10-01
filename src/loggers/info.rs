/// logger `info`
use crate::Config;
use crate::Log;

impl Config {
    pub fn info(&self, content: String) {
        self.write(Log::Info(content));
    }
}
