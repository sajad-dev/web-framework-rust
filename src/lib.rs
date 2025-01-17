use core::router::route_handel;

pub mod core;
pub mod macros;
pub mod run_server;

pub fn bind() {
    // run_server::run_server();
    route_handel::get_routes();
}
