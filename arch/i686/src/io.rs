use core::marker::PhantomData;
use common::Io;

#[derive(Debug)]
pub struct Port<T> {
    pub port: u16,
    phantom: PhantomData<T>
}

impl<T> Port<T> {
    pub const fn new(port: u16) -> Port<T> {
        Port {
            port: port,
            phantom: PhantomData
        }
    }
}

impl Io for Port<u8> {
    type Value = u8;

    fn read(&self) -> u8 {
        let val: u8;

        unsafe {
            asm!("inb %dx, %al"
                : "={al}"(val)
                : "{dx}"(self.port));
        }

        val
    }

    fn write(&mut self, val: u8) {
        unsafe {
            asm!("outb %al, %dx"
                :: "{al}"(val), "{dx}"(self.port));
        }
    }
}
