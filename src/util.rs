#![macro_escape]

#[macro_export]
macro_rules! try_option_or_fail(
    ($e:expr) => (
        match $e {
            Some(x) => x,
            None => fail!("Expected Some, got None")
        }
    );
)

#[macro_export]
macro_rules! try_result_or_fail(
    ($e:expr) => (
        match $e {
            Ok(e) => e,
            Err(e) => fail!(e.desc)
        }
    );
)
