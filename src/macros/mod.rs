#[macro_export]
macro_rules! exception_log {
    ($result:expr,$debug:expr) => {
        match $result {
            Ok(value) => value,
            Err(e) => {
                log::error!("{}", e);
                if $debug {
                    println!("{}", e);
                }
                return;
            }
        }
    };
}
