// logger `init`
// it is for the library's messages
use crate::Config;
use crate::Log;

impl Config {
    pub fn init(&self, content: String) {
        self.write(Log::Init(content));
    }
}
