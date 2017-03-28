pub mod console;

use core::fmt::Write;
use output::console::Console;
use spin::Mutex;

pub static STDOUT: Mutex<Console> = Mutex::new(Console::new());

// NOTE: YAGNI.
pub struct Mirror<'a, T, U>(pub &'a mut T, pub &'a mut U)
        where T: 'a + Write, U: 'a + Write;
