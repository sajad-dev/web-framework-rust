use serde_derive::Deserialize;

use super::get_route::Route;

#[derive(Debug, Deserialize, Clone)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl Method {
    pub fn get_method(&self) -> String {
        match self {
            Method::GET => "get".to_string(),
            Method::POST => "post".to_string(),
            Method::PATCH => "patch".to_string(),
            Method::DELETE => "delete".to_string(),
            Method::PUT => "put".to_string(),
        }
    }
    pub fn get_enum_method(method: &str) -> Method {
        match method {
            "get" => Method::GET,
            "post" => Method::POST,
            "patch" => Method::PATCH,
            "delete" => Method::DELETE,
            "put" => Method::PUT,
            _ => Method::GET,
        }
    }
}

pub enum ErrApi {
    Err404,
    Err405,
    Err403,
    Err402,
    Err401,
    Err500,
}

impl ErrApi {
    pub fn get_err(&self) -> Route {
        match &self {
            ErrApi::Err404 => Route {
                path: "/404".to_string(),
                method: Method::GET,
                controller:"".to_string()
            },
            _ => Route {
                path: "/404".to_string(),
                method: Method::GET,
                controller:"".to_string()
            },
        }
    }
}
