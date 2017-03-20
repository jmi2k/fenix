use core;
use io;

macro_rules! log {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        writeln!(macros::Debug(), $($arg)*);
    }}
}

// Debug helpers

pub struct Debug();

impl core::fmt::Write for Debug {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for char in s.chars() {
            match char {
                '\n' => {
                    serial_out(b'\r');
                    serial_out(b'\n');
                },
                '\x00' ... '\x7f' => serial_out(char as u8),
                _ => {
                    for char in char.escape_default() {
                        serial_out(char as u8)
                    }
                }
            }
        }

        Ok(())
    }
}

fn serial_out(byte: u8) {
    while unsafe { io::in_u8(0x3f8 + 5) & 0x20 == 0 } {}
    unsafe { io::out_u8(0x3f8, byte) };
}
