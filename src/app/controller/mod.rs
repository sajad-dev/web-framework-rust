use web_framework_derive::controller_fn;

pub fn err() -> String {
    String::new()
}

#[controller_fn("home")]
pub fn home_controller(http_request:HashMap<String, String>) -> String {
    return String::from("gf");
}
