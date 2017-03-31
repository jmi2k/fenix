pub mod console;

use core::fmt::{Write, Result};
use device::serial::{Serial, COM1};
use output::console::Console;
use spin::Mutex;

pub static STDOUT: Mutex<Console<Serial>> = Mutex::new(Console::new(&COM1));

pub struct Mirror<'a, T, U>
        where T: 'a + Write, U: 'a + Write {
    pub fst: &'a Mutex<T>,
    pub snd: &'a Mutex<U>
}

impl<'a, T, U> Write for Mirror<'a, T, U>
        where T: 'a + Write, U: 'a + Write {
    fn write_str(&mut self, s: &str) -> Result {
        self.fst
            .lock()
            .write_str(s)?;
        self.snd
            .lock()
            .write_str(s)?;
        Ok(())
    }
}
