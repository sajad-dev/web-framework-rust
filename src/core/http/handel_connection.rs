use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::TcpStream,
};

use crate::core::{
    exception::Exception,
    router::{enums::Method, handel_route::check_route},
};

pub fn handel_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.exception_log().unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let mut status = http_request[0].splitn(3, " ");

    if let (Some(method), Some(route), Some(http_version)) =
        (status.next(), status.next(), status.next())
    {
        let route = check_route(route.to_string(), Method::get_enum_method(method));
        let http_request = get_req(http_request.clone());
        route.run(stream.try_clone().unwrap(),http_request,http_version.to_string());

        // let response = format!("HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\n{}", ou);
        // stream.write_all(response.as_bytes()).unwrap();
    }
}

pub fn get_req<T: ToString>(http_request: Vec<T>) -> HashMap<String, String>
where
    std::string::String: From<T> + Clone + ToString,
{
    let mut ou: HashMap<String, String> = HashMap::new();
    for item in http_request {
        let item_str = item.to_string();
        let mut req = item_str.splitn(2, ":");
        if let (Some(key), Some(value)) = (req.next(), req.next()) {
            ou.insert(key.to_string(), value.to_string());
        }
    }
    ou
}
