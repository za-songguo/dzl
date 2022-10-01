# dzl

A library for logging. It is simple and easy to use. It is also lightweight and has a few dependencies :)

## Usage
1. Add this library as a dependency to your `Cargo.toml`.
2. `src/main.rs`:
```rust
use dzl::Config;

fn main() {
    // not write logs to files
    // let dzl_config = Config::new(false, "dzl.log".to_string());

    // use default config
    let dzl_config = Config::default_config();
    // initializing
    dzl::init(&dzl_config);

    // logging, you can also use `format!()` macro to format the string
    dzl_config.trace("dzl".to_string());
    dzl_config.info("dzl".to_string());
    dzl_config.warn("dzl".to_string());
    dzl_config.error("dzl".to_string());
    // custom log type
    dzl_config.custom("CustomType".to_string(), "dzl".to_string());
 }
```
3. The logs will print to `stdout` or `stderr` *(only for `error` logger)*. If the field `write_logs_to_files` in struct `Config` is false, logs will not write to the log file, they will only print to `stdout` or `stderr`.
The default logging path is `dzl.log`, you can change it with change the field `log_path` in the struct `Config` by use function `new()` with arguments.


**You should make sure that the dictionary of `log_path` exists. The library will only create file `dzl.log`**


**e.g. `log_path: "dzl/dzl.log".to_string()`, the logs will write to file  `dzl/dzl.log`. 😀**