#![no_std]

#[derive(Debug)]
pub struct Settings {
    pub cmdline: &'static str
}

pub trait Io {
    type Value;

    fn read(&self) -> Self::Value;
    fn write(&mut self, val: Self::Value);
}
