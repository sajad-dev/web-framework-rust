use std::env;

use log;

pub trait Exception<T> {
    fn exception_log(self, default: T) -> T;
}

pub fn err(e: String) {
    log::error!("{}", e);
    if env::var("DEBUG").unwrap_or_else(|_| "false".to_string()) == "true" {
        println!("{}", e);
    }
}

impl<T> Exception<T> for Result<T, &str> {
    fn exception_log(self, default: T) -> T {
        match self {
            Ok(value) => value,
            Err(e) => {
                err(e.to_string());
                return default;
            }
        }
    }
}
