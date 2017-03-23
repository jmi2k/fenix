#![feature(lang_items, naked_functions)]
#![no_std]

// Fenix kernel
//
// NOTES:
//  - .::macros::log : Improve implementation.

extern crate rlibc;
extern crate multiboot2;

#[macro_use]
mod macros;
mod arch;

fn kmain(settings: &arch::Settings) {
    log!("cmdline: {:?}", settings.cmdline);
}

#[naked]
#[no_mangle]
pub unsafe fn start() -> ! {
    let settings = arch::start();

    kmain(&settings);
    panic!("`main` returned!");
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic(fmt: core::fmt::Arguments,
        file: &'static str, line: u32) -> ! {
    log!("Fenix panicked at file: {}, line: {}; and flew away...", file,
        line);
    log!("... but he left a message before leaving:");
    log!("");
    log!("{}", fmt);

    loop {}
}
