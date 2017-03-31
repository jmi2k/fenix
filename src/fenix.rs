#![feature(asm, const_fn, lang_items, naked_functions)]
#![no_std]

#[macro_use]
extern crate bitflags;

extern crate multiboot2;
extern crate rlibc;
extern crate spin;

#[macro_use]
mod macros;
mod device;
mod output;
mod io;

#[allow(improper_ctypes)]
extern { static stack: (); }

fn kmain(cmdline: &'static str) {
    logln!("cmdline: {:?}", cmdline);
}

#[naked]
#[no_mangle]
pub fn start() -> ! {
    let magic: u32;
    let addr: usize;

    unsafe {
        asm!("\
            mov %eax, $0
            mov %ebx, $1"
            : "=r"(magic), "=r"(addr)
            :: "eax", "ebx");

        asm!("mov $0, %esp"
            :: "r"(&stack as *const _)
            : "esp");
    }

    if magic != 0x36d76289 { panic!("Multiboot data not found") }

    let info = unsafe { multiboot2::load(addr) };
    let memory_map = info.memory_map_tag()
        .expect("Memory map tag not found")
        .memory_areas();
    let kernel_sections = info.elf_sections_tag()
        .expect("ELF sections tag not found")
        .sections();
    let cmdline = info.command_line_tag()
        .map_or("", |tag| tag.command_line());

    kmain(cmdline);
    panic!("`kmain` returned");
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic(fmt: core::fmt::Arguments,
        file: &'static str, line: u32) -> ! {
    logln!("Fenix panicked at file: {}, line: {}; and flew away...", file,
           line);
    logln!("... but he left a message before leaving:");
    logln!("");
    logln!("{}", fmt);

    loop {}
}
