use dtable::idt::IDT;

#[macro_export]
macro_rules! int {
    ($num:expr) => (asm!("int $0" :: "N"($num)))
}

unsafe extern "x86-interrupt" fn unbound(_: &mut (), _: ()) {
    warn!("unbound interrupt")
}

#[inline]
pub fn enable() {
    unsafe { asm!("sti") }
}

#[inline]
pub fn disable() {
    unsafe { asm!("cli") }
}

pub unsafe fn init() {
    let mut idt = IDT.lock();

    idt.divide_by_zero.set_func(unbound);
    idt.debug.set_func(unbound);
    idt.non_maskable_interrupt.set_func(unbound);
    idt.breakpoint.set_func(unbound);
    idt.overflow.set_func(unbound);
    idt.bound_range_exceeded.set_func(unbound);
    idt.invalid_opcode.set_func(unbound);
    idt.device_not_available.set_func(unbound);
    idt.double_fault.set_func(unbound);
    idt.invalid_tss.set_func(unbound);
    idt.segment_not_present.set_func(unbound);
    idt.ss_fault.set_func(unbound);
    idt.general_protection_fault.set_func(unbound);
    idt.page_fault.set_func(unbound);
    idt.x87_fpe.set_func(unbound);
    idt.alignment_check.set_func(unbound);
    idt.machine_check.set_func(unbound);
    idt.simd_fpe.set_func(unbound);
    idt.virtualization.set_func(unbound);

    for i in 0..224 {
        idt.interrupts[i].set_func(unbound);
    }

    idt.load();
    enable()
}
