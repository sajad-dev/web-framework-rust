use web_framework_derive::controller_fn;

#[controller_fn("home")]
pub fn home_controller() -> String {
    return String::from("gf");
}
