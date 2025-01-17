pub mod core;
pub mod macros;
pub mod run_server;
pub mod utils;

pub fn bind() {
    run_server::run_server();
}
