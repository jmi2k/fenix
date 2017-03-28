use core::fmt::{Write, Result};
use spin::Mutex;

// TODO: Using Mutex is not very appropiate, something more generic is better.
pub struct Console<'a, T> where T: 'a + Write {
    pub enable_listchars: bool,
    pub output: &'a Mutex<T>
}

impl<'a, T> Console<'a, T> where T: 'a + Write {
    pub const fn new(out: &'a Mutex<T>) -> Self {
        Self {
            enable_listchars: false,
            output: out
        }
    }
}

impl<'a, T> Write for Console<'a, T> where T: 'a + Write {
    fn write_str(&mut self, s: &str) -> Result {
        let mut output = self.output.lock();

        if ! self.enable_listchars { output.write_str(s) }
        else {
            for char in s.chars() {
                match char {
                    '\n' => output.write_str("¬\n"),
                    '\t' => output.write_char('»'),
                    ' ' => output.write_char('·'),
                    _ => output.write_char(char)
                }?;
            }

            Ok(())
        }
    }
}
