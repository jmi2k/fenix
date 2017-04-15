#![feature(abi_x86_interrupt, asm, const_fn, lang_items, naked_functions)]
#![no_std]

#[macro_use]
extern crate bitflags;

extern crate bit;
extern crate rlibc;
extern crate spin;

#[macro_use]
mod log;

#[macro_use]
mod interrupt;

mod device;
mod dtable;
mod io;

#[allow(improper_ctypes)]
#[allow(dead_code)]
extern { static stack: (); }

fn kmain() -> ! {
    info!("Reached `kmain`...");
    loop {}
}

#[naked]
#[no_mangle]
pub unsafe fn start() -> ! {
    asm!("mov $0, %esp"
            :: "r"(&stack));

    device::serial::init();
    interrupt::init();

    kmain();
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic(fmt: core::fmt::Arguments,
        file: &'static str, line: u32) -> ! {
    let level = log::Level::Panic;

    log!(level, "| Fenix panicked at {}:{} |", file, line);
    log!(level);
    log!(level, "{}", fmt);

    loop {}
}
