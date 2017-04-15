use core::marker::PhantomData;

#[derive(Debug)]
pub struct Port<T> {
    pub port: u16,
    phantom: PhantomData<T>
}

#[derive(Debug)]
pub struct ReadOnly<T> where T: Io { inner: T }

#[derive(Debug)]
pub struct WriteOnly<T> where T: Io { inner: T }

pub trait Input {
    type Value;

    fn read(&mut self) -> Self::Value;
}

pub trait Output {
    type Value;

    fn write(&mut self, val: Self::Value);
}

pub trait Io: Input + Output {}

impl<T> Port<T> {
    pub const fn new(port: u16) -> Self {
        Self {
            port: port,
            phantom: PhantomData
        }
    }
}

impl Input for Port<u8> {
    type Value = u8;

    fn read(&mut self) -> Self::Value {
        let val: Self::Value;

        unsafe {
            asm!("inb %dx, %al"
                    : "={al}"(val)
                    : "{dx}"(self.port));
        }

        val
    }
}

impl Output for Port<u8> {
    type Value = u8;

    fn write(&mut self, val: Self::Value) {
        unsafe {
            asm!("outb %al, %dx"
                    :: "{al}"(val), "{dx}"(self.port))
        }
    }
}

impl<T> ReadOnly<T> where T: Io {
    pub const fn new(inner: T) -> Self {
        Self { inner: inner }
    }
}

impl<T> Input for ReadOnly<T> where T: Io {
    type Value = <T as Input>::Value;

    fn read(&mut self) -> <T as Input>::Value {
        self.inner.read()
    }
}

impl<T> Output for WriteOnly<T> where T: Io {
    type Value = <T as Output>::Value;

    fn write(&mut self, val: <T as Output>::Value) {
        self.inner.write(val)
    }
}

impl<T> Io for T where T: Input + Output {}
