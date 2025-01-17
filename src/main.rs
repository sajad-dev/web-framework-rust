use web_framework_rust::bind;

extern crate log;
extern crate simplelog;

use simplelog::*;
use std::fs::OpenOptions;

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            OpenOptions::new()
                .create(true)
                .append(true)
                .open("log/server.log")
                .unwrap(),
        ),
    ])
    .unwrap();

    dotenv::dotenv().ok();

    bind();
}
