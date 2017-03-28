pub mod console;

use core::fmt::{Write, Result};
use device::serial::{Serial, COM1};
use output::console::Console;
use spin::Mutex;

pub static STDOUT: Mutex<Console<Serial>> = Mutex::new(Console::new(&COM1));

// TODO: Using Mutex is not very appropiate, something more generic is better.
pub struct Mirror<'a, T, U>(pub &'a Mutex<T>, pub &'a Mutex<U>)
        where T: 'a + Write, U: 'a + Write;

impl<'a, T, U> Write for Mirror<'a, T, U>
        where T: 'a + Write, U: 'a + Write {
    fn write_str(&mut self, s: &str) -> Result {
        self.0.lock().write_str(s)?;
        self.1.lock().write_str(s)?;
        Ok(())
    }
}
