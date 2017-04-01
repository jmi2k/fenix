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
mod io;
mod output;

#[allow(improper_ctypes)]
extern { static stack: (); }

static mut MULTIBOOT_INFO: Option<&multiboot2::BootInformation> = None;

fn kmain() {
    logln!("cmdline: {:?}", cmdline());
}

#[naked]
#[no_mangle]
pub unsafe fn start() -> ! {
    let magic: u32;
    let addr: usize;

    asm!("\
        mov %eax, $0
        mov %ebx, $1"
        : "=r"(magic), "=r"(addr)
        :: "eax", "ebx");

    asm!("mov $0, %esp"
        :: "r"(&stack as *const _)
        : "esp");

    if magic != 0x36d76289 { panic!("multiboot info not found") }
    MULTIBOOT_INFO = Some(multiboot2::load(addr));

    kmain();
    panic!("`kmain` returned");
}

fn memory_map() -> multiboot2::MemoryAreaIter {
    unsafe {
        MULTIBOOT_INFO.unwrap()
            .memory_map_tag()
            .expect("memory map tag not found")
            .memory_areas()
    }
}

fn kernel_sections() -> multiboot2::ElfSectionIter {
    unsafe {
        MULTIBOOT_INFO.unwrap()
            .elf_sections_tag()
            .expect("kernel sections tag not found")
            .sections()
    }
}

fn cmdline() -> &'static str {
    unsafe {
        MULTIBOOT_INFO.unwrap()
            .command_line_tag()
            .map_or("", |tag| tag.command_line())
    }
}

#[lang = "panic_fmt"]
#[no_mangle]
extern fn panic(fmt: core::fmt::Arguments,
        file: &'static str, line: u32) -> ! {
    logln!("| Fenix panicked at {}:{} |", file, line);
    logln!();
    logln!("{}", fmt);

    loop {}
}
