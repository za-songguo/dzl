/// logger `trace`
use crate::Config;
use crate::Log;

impl Config {
    pub fn trace(&self, content: String) {
        self.write(Log::Trace(content));
    }
}
