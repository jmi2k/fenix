use core::fmt::{Write, Result};
use io::{ReadOnly, Port, Io};
use spin::Mutex;

pub static COM1: Mutex<Serial> = Mutex::new(Serial::new(0x3f8));
pub static COM2: Mutex<Serial> = Mutex::new(Serial::new(0x2f8));

bitflags! {
    pub flags Lsr: u8 {
        const INPUT_EMPTY = 1,
        const OUTPUT_EMPTY = 1 << 5
    }
}

#[derive(Debug)]
pub struct Serial {
    data: Port<u8>,
    int_enable: Port<u8>,
    fifo_ctrl: Port<u8>,
    line_ctrl: Port<u8>,
    modem_ctrl: Port<u8>,
    line_status: ReadOnly<Port<u8>>,
    modem_status: ReadOnly<Port<u8>>
}

impl Serial {
    const fn new(base: u16) -> Self {
        Self {
            data: Port::new(base),
            int_enable: Port::new(base + 1),
            fifo_ctrl: Port::new(base + 2),
            line_ctrl: Port::new(base + 3),
            modem_ctrl: Port::new(base + 4),
            line_status: ReadOnly::new(Port::new(base + 5)),
            modem_status: ReadOnly::new(Port::new(base + 6))
        }
    }

    pub fn write(&mut self, byte: u8) {
        while ! self.line_status().contains(OUTPUT_EMPTY) {}
        self.data.write(byte);
    }

    pub fn line_status(&self) -> Lsr {
        Lsr::from_bits_truncate(self.line_status.read())
    }
}

impl Write for Serial {
    fn write_str(&mut self, s: &str) -> Result {
        for byte in s.bytes() { self.write(byte) }
        Ok(())
    }
}
