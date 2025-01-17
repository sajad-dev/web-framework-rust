use std::{env, panic::Location, time::SystemTime};

use chrono::Local;
use log;

pub trait Exception<T> {
    fn exception_log(self) -> Option<T>;
}

pub fn err(e: String, location: &Location) {
    let local = Local::now();
    log::error!(
        "{} -> file: {} , line: {}, time : {:?}",
        e,
        location.file(),
        location.line(),
        local.format("%Y-%m-%d %H:%M:%S").to_string()
    );
    if env::var("DEBUG").unwrap_or_else(|_| "false".to_string()) == "true" {
        println!("{}", e);
    }
}

impl<T, E> Exception<T> for Result<T, E>
where
    E: std::fmt::Debug + ToString,
{
    fn exception_log(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(e) => {
                err(e.to_string(), Location::caller());
                eprintln!("Error: {:?}", e);
                None
            }
        }
    }
}
