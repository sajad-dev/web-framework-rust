pub mod app;
pub mod core;
pub mod macros;
pub mod routes;

pub fn bind() {
    core::http::run_server();
}
