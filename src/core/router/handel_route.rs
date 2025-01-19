// use 
use super::{
    enums::{ErrApi, Method},
    get_route::{get_one_route, Route},
};

impl Route {
    pub fn middleware() {}

    pub fn controller_fn(&self) {
        
    }

    pub fn run(&self) {
        
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
