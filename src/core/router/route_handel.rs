use crate::core::exception::Exception;
use std::{env, fs};

pub fn get_routes() {
    let dir = env::var("CARGO_MANIFEST_DIR")
        .exception_log()
        .unwrap_or_else(|| "".to_string());

    let dir = format!("{}/{}", dir, "/src/routes/api.toml".to_string());

    // println!("{}", dir);
    let file = fs::read_to_string(dir)
        .exception_log()
        .unwrap_or_else(|| "".to_string());

    
}
