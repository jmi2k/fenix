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
extern { static bss: (); }

fn kmain(cmdline: &'static str) {
    logln!("cmdline: {:?}", cmdline);
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
        :: "r"(&bss as *const _)
        : "esp");

    assert_eq!(magic, 0x36d76289);

    let info = multiboot2::load(addr);
    let memory_map_tag = info.memory_map_tag()
        .expect("Memory map tag not found");
    let kernel_sections_tag = info.elf_sections_tag()
        .expect("ELF sections tag not found");
    let cmdline = info.command_line_tag()
        .map_or("", |tag| tag.command_line());
    let string_table = kernel_sections_tag.string_table();

    logln!("memory_map:");
    for area in memory_map_tag.memory_areas() {
        logln!("    addr: 0x{:08x}, length: 0x{:08x}",
               area.base_addr, area.length);
    }

    logln!("kernel_sections:");
    for section in kernel_sections_tag.sections() {
        logln!("    {}:", string_table.section_name(section));
        logln!("        addr: 0x{:08x}", section.addr);
        logln!("        size: {}", section.size);
    }

    kmain(cmdline);
    panic!("`main` returned");
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
