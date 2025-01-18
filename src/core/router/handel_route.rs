use super::get_route::{get_one_route, Route};

impl Route {
    pub fn middleware() {}

    pub fn run() {}
}

pub fn check_route(path: String) -> Route {
    match get_one_route(path) {
        Some(value) => value,
        None => Route { path: "404".to_string() },
    }
}
