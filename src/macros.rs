#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        arch::stdout().map(|out| writeln!(out, $($arg)*));
    }}
}
