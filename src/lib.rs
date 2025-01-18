use core::router::get_route;

pub mod core;
pub mod macros;
pub mod run_server;

pub fn bind() {
    run_server::run_server();
}
