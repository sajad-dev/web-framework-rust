use crate::core::exception::Exception;
use serde_derive::Deserialize;
use std::{collections::HashMap, env, fs};

#[derive(Debug, Deserialize, Clone)]

pub struct Route {
    pub path: String,
}

pub struct Routes {
    pub api: Vec<Route>,
}

pub fn get_routes() -> Routes {
    let dir = env::var("CARGO_MANIFEST_DIR")
        .exception_log()
        .unwrap_or_else(|| "".to_string());

    let dir = format!("{}/{}", dir, "/src/routes/api.toml".to_string());

    let file = fs::read_to_string(dir)
        .exception_log()
        .unwrap_or_else(|| "".to_string());

    let routes: HashMap<String, Vec<Route>> = toml::de::from_str(&file)
        .exception_log()
        .unwrap_or_else(|| HashMap::new());

    let key = routes.get("api").exception_log().unwrap().clone();

    Routes { api: key }
}

pub fn get_one_route(path: String) -> Option<Route> {
    for route in get_routes().api {
        if path == route.path {
            return Some(route);
        }
    }
    return None;
}
