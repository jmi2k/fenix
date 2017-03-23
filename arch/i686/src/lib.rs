#![feature(asm, const_fn, naked_functions)]
#![no_std]

// Fenix i686 backend
//
// NOTES:
//  - .::stdout : Avoid usage of `unsafe`.
//  - .::io::*  : `dx` could be replaced with something more generic.
//
//  - .::device::serial::{COM1, COM2} :
//    Remove `mut` and use `Mutex`.

extern crate common;

#[macro_use]
extern crate bitflags;
extern crate multiboot2;

mod device;
mod io;

use common::Settings;

extern { static bss: (); }

#[inline(always)]
#[naked]
pub unsafe fn start() -> Settings {
    let magic: u32;
    let addr: usize;

    asm!("\
        mov %eax, $0
        mov %ebx, $1"
        : "=r"(magic), "=r"(addr)
        :: "eax", "ebx");

    asm!("mov $0, %esp"
        :: "r"(&bss as *const _)
        : "esp");

    assert_eq!(magic, 0x36d76289);

    let info = multiboot2::load(addr);

    stdout()
        .unwrap()
        .init();

    // log!("Done!");

    Settings {
        cmdline: info.command_line_tag()
            .map_or("", |tag| tag.command_line())
    }
}

#[inline]
pub fn stdout() -> Option<&'static mut device::serial::Serial> { 
    unsafe { Some(&mut device::serial::COM1) }
}
