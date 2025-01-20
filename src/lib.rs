pub mod app;
pub mod core;
pub mod macros;

pub fn bind() {
    core::http::run_server();
}
