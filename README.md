# dzl

A library for logging. It is simple and easy to use. It is also lightweight and has a few dependencies :)

## Usage

1. Add this library as a dependency to your `Cargo.toml`.
2. `src/main.rs`:

```rust
use dzl::Config;

fn main() {
    // not write logs to files
    // let dzl_config = Config::new(false, true, "dzl.log".to_string());

    // without color and emojis
    // let dzl_config = Config::new(true, false, "dzl.log".to_string());
    // use default config
    let dzl_config = Config::default_config();
    // initializing
    dzl::init(&dzl_config);

    // logging, you can also use `format!()` macro to format the string
    dzl_config.trace("dzl".to_string());
    dzl_config.info("dzl".to_string());
    dzl_config.debug("debug".to_string());
    dzl_config.warn("dzl".to_string());
    dzl_config.error("dzl".to_string());
    // custom log type
    dzl_config.custom("CustomType".to_string(), "dzl".to_string());
 }
```

3. The logs will print to `stdout` or `stderr` _(only for `error` logger)_. If the field `write_logs_to_files` in struct `Config` is false, logs will not write to the log file, they will only print to `stdout` or `stderr`.
   The default logging path is `dzl.log`, you can change it with change the field `log_path` in the struct `Config` by using function `new()` with arguments.

**You should make sure that the dictionary of `log_path` exists. The library will only create file `dzl.log`**

**e.g. `log_path: "dzl/dzl.log".to_string()`, the logs will write to file `dzl/dzl.log`. 😀**

### Output

With color and emojis:

_(In the terminal which supports color display, you will see the output with color)_

```[dzl 😀] Log Type: Init, Time: 2022-10-03 13:15:14.626647283 +08:00:00, message: Called init() function
[dzl 💬] Log Type: Trace, Time: 2022-10-03 13:15:14.628071688 +08:00:00, message: dzl
[dzl 💬] Log Type: Info, Time: 2022-10-03 13:15:14.62999558 +08:00:00, message: dzl
[dzl 💬] Log Type: Debug, Time: 2022-10-03 13:15:14.631733951 +08:00:00, message: dzl
[dzl 😐] Log Type: Warn, Time: 2022-10-03 13:15:14.633433007 +08:00:00, message: dzl
[dzl 😕] Log Type: Error, Time: 2022-10-03 13:15:14.635226656 +08:00:00, message: dzl
[dzl 🌱] Log Type: Nothing, Time: 2022-10-03 13:15:14.63703675 +08:00:00, message: dzl
```
