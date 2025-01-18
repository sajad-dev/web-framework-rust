// use std::backtrace::Backtrace;
use std::env;

use backtrace::{Backtrace, BacktraceSymbol};
use chrono::Local;
use log;

pub trait Exception<T> {
    fn exception_log(self) -> Option<T>;
}

pub fn err(e: String, backtrace: BacktraceSymbol) {
    let local = Local::now();
    log::error!(
        "{} -> File : {:?}, Line: {:?}, -- {:?} --, time :{}",
        e,
        backtrace.filename().unwrap(),
        backtrace.lineno().unwrap(),
        backtrace.name().unwrap(),
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
                let backtrace2 = Backtrace::new();
                let frame = backtrace2.frames().get(1).unwrap();
                let frame = frame.symbols().get(0).unwrap().clone();

                err(e.to_string(), frame);
                eprintln!("Error: {:?}", e);
                None
            }
        }
    }
}

impl<T> Exception<T> for Option<T> {
    fn exception_log(self) -> Option<T> {
        match self {
            Some(value) => Some(value),
            None => {
                let backtrace2 = Backtrace::new();
                let frame = backtrace2.frames().get(1).unwrap();
                let frame = frame.symbols().get(0).unwrap().clone();

                err("None Value".to_string(), frame);
                None
            }
        }
    }
}
