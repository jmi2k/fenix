use core::fmt::{Result, Write};
use spin::Mutex;

pub struct Console<'a, T> where T: 'a + Write { pub output: &'a Mutex<T> }

impl<'a, T> Console<'a, T> where T: 'a + Write {
    pub const fn new(out: &'a Mutex<T>) -> Self {
        Self { output: out }
    }
}

impl<'a, T> Write for Console<'a, T> where T: 'a + Write {
    fn write_str(&mut self, s: &str) -> Result {
        self.output
            .lock()
            .write_str(s)
    }
}
