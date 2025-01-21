use std::{collections::HashMap, io::Write, net::TcpStream};


use crate::{
    core::utils::response::{ response, Status}, routes::provider,
};

use super::{
    enums::{ErrApi, Method},
    get_route::{get_one_route, Route},
};

impl Route {
    pub fn middleware() {}

    pub fn controller_fn(&self, http_request: HashMap<String, String>) -> String {
        provider(&self.controller)(http_request)
    }

    pub fn run(
        &self,
        mut stream: TcpStream,
        http_request: HashMap<String, String>,
        _http_version: String,
    ) {
        let content = self.controller_fn(http_request);
        let res = response(Status::OK, "application/json", &content, HashMap::new());

        stream.write_all(res.as_bytes()).unwrap();
    }
}

pub fn check_route(path: String, method: Method) -> Route {
    match get_one_route(path) {
        Some(value) => {
            if value.method.get_method() != method.get_method() {
                return ErrApi::Err405.get_err();
            }
            value
        }
        None => ErrApi::Err404.get_err(),
    }
}
