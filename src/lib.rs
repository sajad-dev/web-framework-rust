pub mod app;
pub mod core;
pub mod macros;

pub fn bind() {
    core::run_server::run_server();
}
