use crate::Config;
use crate::Log;

impl Config {
    // logger `init`
    // it is for the library's messages
    pub fn init(&self, content: String) {
        self.write(Log::Init(content));
    }
}
