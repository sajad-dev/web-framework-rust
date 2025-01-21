use std::collections::HashMap;

use crate::app::controller::*;

type FnType = fn(http_request: HashMap<String, String>) -> String;
pub fn provider(name: &str) -> FnType {
    match name {
        "home" => home_controller,
        _ => err,
    }
}
