use core::fmt::{Write, Result};
use device;

// NOTE: Maybe parametizing Console over types implementing the (or something
//       alike) is a good idea in the future. Now, as traits can't be
//       implemented in external types (such as spin::Mutex. as I was planning)
//       it makes very little sense, so we assume a default output in the
//       implementation.
pub struct Console {
    pub enable_listchars: bool
}

impl Console {
    pub const fn new() -> Self {
        Self { enable_listchars: false }
    }
}

impl Write for Console {
    fn write_str(&mut self, s: &str) -> Result {
        let mut output = device::serial::COM1.lock();

        if ! self.enable_listchars { output.write_str(s) }
        else {
            for char in s.chars() {
                match char {
                    '\n' => output.write_str("¬\n"),
                    '\t' => output.write_char('»'),
                    ' ' => output.write_char('·'),
                    _ => output.write_char(char)
                };
            }

            Ok(())
        }
    }
}
