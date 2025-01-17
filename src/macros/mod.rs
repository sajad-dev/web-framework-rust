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

#[macro_export]
macro_rules! create_struct {
    ($name:ident,$($field:ident: $type:ty),*) => {
       pub struct $name {
            $(
                $field: $type,
            )*
        }
    };
}
