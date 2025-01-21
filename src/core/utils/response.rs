use std::collections::HashMap;

pub enum Status {
    OK,
}

impl Status {
    fn get_status(&self) -> &str {
        match self {
            Status::OK => "HTTP/1.1 200 OK",
        }
    }
}

pub fn response(
    status: Status,
    content_type: &str,
    content: &str,
    header_user: HashMap<&str, &str>,
) -> String {
    let mut headers = HashMap::new();

    let leng = content.len().to_string();
    headers.insert("Content-Type", content_type);
    headers.insert("Content-Length", &leng);
    headers.insert("Cache-Control", "no-cache, no-store, must-revalidate");
    headers.insert("Access-Control-Allow-Origin", "*");


    headers.extend(header_user);

    let mut  res=String::new();
    for (key, value) in headers {
        res = res.to_string() + &format!("{}:{}\r\n", key, value);
    }
    let res = status.get_status().to_string() + "\r\n" + &res + "\r\n" + &content;

    return res;
}
