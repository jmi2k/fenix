use core::fmt::{Write, Result};
use io::Port;
use common::Io;

pub static mut COM1: Serial = Serial::new(0x3f8);
pub static mut COM2: Serial = Serial::new(0x3e8);

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

    // NOTE: These are read-only.
    line_status: Port<u8>,
    modem_status: Port<u8>
}

impl Serial {
    const fn new(base: u16) -> Serial {
        Serial {
            data: Port::new(base),
            int_enable: Port::new(base + 1),
            fifo_ctrl: Port::new(base + 2),
            line_ctrl: Port::new(base + 3),
            modem_ctrl: Port::new(base + 4),
            line_status: Port::new(base + 5),
            modem_status: Port::new(base + 6)
        }
    }

    pub fn init(&mut self) {
        self.int_enable.write(0);
        self.line_ctrl.write(0x80);
        self.data.write(0x03);
        self.int_enable.write(0x00);
        self.line_ctrl.write(0x03);
        self.fifo_ctrl.write(0xc7);
        self.modem_ctrl.write(0x0b);
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
        for byte in s.bytes() {
            self.write(byte)
        }

        Ok(())
    }
}
