use core::marker::PhantomData;

#[derive(Debug)]
pub struct Port<T> {
    pub port: u16,
    phantom: PhantomData<T>
}

#[derive(Debug)]
pub struct ReadOnly<T> where T: Io {
    pub inner: T
}

#[derive(Debug)]
pub struct WriteOnly<T> where T: Io {
    pub inner: T
}

pub trait Io {
    type Value;

    fn read(&self) -> Self::Value;
    fn write(&mut self, val: Self::Value);
}

impl<T> Port<T> {
    pub const fn new(port: u16) -> Self {
        Self {
            port: port,
            phantom: PhantomData
        }
    }
}

impl Io for Port<u8> {
    type Value = u8;

    fn read(&self) -> Self::Value {
        let val: Self::Value;

        unsafe {
            asm!("inb %dx, %al"
                : "={al}"(val)
                : "{dx}"(self.port));
        }

        val
    }

    fn write(&mut self, val: Self::Value) {
        unsafe {
            asm!("outb %al, %dx"
                :: "{al}"(val), "{dx}"(self.port));
        }
    }
}

impl<T> ReadOnly<T> where T: Io {
    pub const fn new(inner: T) -> Self {
        Self { inner: inner }
    }

    pub fn read(&self) -> T::Value {
        self.inner.read()
    }
}

impl<T> WriteOnly<T> where T: Io {
    pub const fn new(inner: T) -> Self {
        Self { inner: inner }
    }

    pub fn write(&mut self, val: T::Value) {
        self.inner.write(val)
    }
}
