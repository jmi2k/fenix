use core::marker::PhantomData;
use bit::BitIndex;
use dtable::{Dtr, Privilege};
use spin::Mutex;

pub static IDT: Mutex<Idt> = Mutex::new(Idt::new());

bitflags! {
    pub flags Attr: u8 {
        const PRESENT = 0b10000000,
        const PRIVILEGE = 0b1100000,
        const SS = 0b10000,
        const TYPE = 0b1111
    }
}

#[repr(C, packed)]
pub struct Idt {
    pub divide_by_zero: Entry,
    pub debug: Entry,
    pub non_maskable_interrupt: Entry,
    pub breakpoint: Entry,
    pub overflow: Entry,
    pub bound_range_exceeded: Entry,
    pub invalid_opcode: Entry,
    pub device_not_available: Entry,
    pub double_fault: Entry,
    unused: Entry,  // Coprocessor Segment Overrun
    pub invalid_tss: Entry,
    pub segment_not_present: Entry,
    pub ss_fault: Entry,
    pub general_protection_fault: Entry,
    pub page_fault: Entry,
    reserved_1: Entry,
    pub x87_fpe: Entry,
    pub alignment_check: Entry,
    pub machine_check: Entry,
    pub simd_fpe: Entry,
    pub virtualization: Entry,
    reserved_2: [Entry; 9],
    pub security_exception: Entry,
    reserved_3: Entry,

    pub interrupts: [Entry; 224],
}

pub type Handler<T> = unsafe extern "x86-interrupt" fn (&mut (), T);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Gate {
    Task = 0b101,
    Interrupt = 0b1110,
    Trap = 0b1111
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C, packed)]
pub struct Entry<T = ()> where T: Eq + PartialEq {
    offset_1: u16,
    pub selector: u16,
    zero: u8,
    attr: Attr,
    offset_2: u16,
    phantom: PhantomData<T>
}

impl Idt {
    pub const fn new() -> Self {
        Self {
            divide_by_zero: Entry::new(),
            debug: Entry::new(),
            non_maskable_interrupt: Entry::new(),
            breakpoint: Entry::new(),
            overflow: Entry::new(),
            bound_range_exceeded: Entry::new(),
            invalid_opcode: Entry::new(),
            device_not_available: Entry::new(),
            double_fault: Entry::new(),
            unused: Entry::new(),
            invalid_tss: Entry::new(),
            segment_not_present: Entry::new(),
            ss_fault: Entry::new(),
            general_protection_fault: Entry::new(),
            page_fault: Entry::new(),
            reserved_1: Entry::new(),
            x87_fpe: Entry::new(),
            alignment_check: Entry::new(),
            machine_check: Entry::new(),
            simd_fpe: Entry::new(),
            virtualization: Entry::new(),
            reserved_2: [Entry::new(); 9],
            security_exception: Entry::new(),
            reserved_3: Entry::new(),

            interrupts: [Entry::new(); 224],
        }
    }

    pub unsafe fn load(&self) {
        use core::mem::size_of;

        let dtr = Dtr {
            limit: size_of::<Self>() as u16 - 1,
            base: self as *const _
        };

        asm!("lidt ($0)"
                :: "r"(&dtr)
                : "memory");
    }
}

impl<T> Entry<T> where T: Eq + PartialEq {
    pub const fn new() -> Self {
        Self {
            offset_1: 0,
            selector: 0x10,
            zero: 0,
            attr: Attr { bits: 0 },
            offset_2: 0,
            phantom: PhantomData
        }
    }

    pub fn privilege(&self) -> Privilege {
        match self.attr.bits().bit_range(5..7) {
            0 => Privilege::Ring0,
            1 => Privilege::Ring1,
            2 => Privilege::Ring2,
            3 => Privilege::Ring3,
            _ => unreachable!()
        }
    }

    pub fn gate(&self) -> Gate {
        match self.attr.bits().bit_range(0..4) {
            0b101 => Gate::Task,
            0b1110 => Gate::Interrupt,
            0b1111 => Gate::Trap,
            t => panic!("invalid interrupt gate {:#04b}", t)
        }
    }

    pub fn offset(&self) -> u32 {
        *0u32
            .set_bit_range(0..16, self.offset_1 as _)
            .set_bit_range(16..32, self.offset_2 as _)
    }

    pub fn set_privilege(&mut self, privilege: Privilege) {
        self.attr.bits.set_bit_range(5..7, privilege as _);
    }

    pub fn set_gate(&mut self, gate: Gate) {
        self.attr.bits.set_bit_range(0..4, gate as _);
    }

    pub fn set_offset(&mut self, offset: u32) {
        self.offset_1 = offset.bit_range(0..16) as _;
        self.offset_2 = offset.bit_range(16..32) as _
    }

    pub fn set_func(&mut self, func: Handler<T>) {
        self.attr.insert(PRESENT);

        self.set_privilege(Privilege::Ring0);
        self.set_gate(Gate::Interrupt);
        self.set_offset(func as _)
    }
}
