use device::serial::{Serial, COM1};
use device::console::Console;
use core::fmt::{Display, Formatter, Result};
use spin::Mutex;

pub static STDOUT: Mutex<Console<Serial>> = Mutex::new(Console::new(&COM1));

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!(::log::STDOUT.lock(), $($arg)*);
    }}
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*))
}

#[macro_export]
macro_rules! log {
    ($lvl:expr) => (println!("{}", $lvl));
    ($lvl:expr, $fmt:expr) => (println!("{} {}", $lvl, format_args!($fmt)));
    ($lvl:expr, $fmt:expr, $($arg:tt)*) => {
        print!("{} ", $lvl);
        println!($fmt, $($arg)*)
    }
}

#[macro_export]
macro_rules! info {
    () => (log!(::log::Level::Info));
    ($fmt:expr) => (log!(::log::Level::Info, $fmt));
    ($fmt:expr, $($arg:tt)*) => (log!(::log::Level::Info, $fmt, $($arg)*));
}

#[macro_export]
macro_rules! warn {
    () => (log!(::log::Level::Warning));
    ($fmt:expr) => (log!(::log::Level::Warning, $fmt));
    ($fmt:expr, $($arg:tt)*) => (log!(::log::Level::Warning, $fmt, $($arg)*));
}

#[macro_export]
macro_rules! err {
    () => (log!(::log::Level::Error));
    ($fmt:expr) => (log!(::log::Level::Error, $fmt));
    ($fmt:expr, $($arg:tt)*) => (log!(::log::Level::Error, $fmt, $($arg)*));
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Level {
    Info,
    Warning,
    Error,
    Panic
}

impl Display for Level {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match *self {
            Level::Info => write!(fmt, "[Info]"),
            Level::Warning => write!(fmt, "[Warning]"),
            Level::Error => write!(fmt, "[Error]"),
            Level::Panic => write!(fmt, "[Panic]")
        }
    }
}
