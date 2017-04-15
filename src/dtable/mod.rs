pub mod idt;

#[derive(Debug)]
#[repr(C, packed)]
pub struct Dtr<T> {
    pub limit: u16,
    pub base: *const T
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Privilege {
    Ring0 = 0,
    Ring1 = 1,
    Ring2 = 2,
    Ring3 = 3
}
