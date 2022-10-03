
use crate::Config;
use crate::Log;

impl Config {
    /// logger `trace`
    pub fn trace(&self, content: String) {
        self.write(Log::Trace(content));
    }
}
