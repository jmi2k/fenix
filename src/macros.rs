#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!(::output::STDOUT.lock(), $($arg)*);
    }}
}

macro_rules! logln {
    () => (log!("\n"));
    ($fmt:expr) => (log!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (log!(concat!($fmt, "\n"), $($arg)*))
}
