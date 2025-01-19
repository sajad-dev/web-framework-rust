use crate::{app, core::exception::Exception};

// use
use super::{
    enums::{ErrApi, Method},
    get_route::{get_one_route, Route},
};

impl Route {
    pub fn middleware() {}

    pub fn controller_fn(&self) -> String {
        app::controller::controller_fn_hashmap()
            .get(&self.controller)
            .exception_log()
            .unwrap()()
        // Fix unwarp and falback

    }

    pub fn run(&self) -> String {
        self.controller_fn()
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
