#![feature(asm, lang_items, naked_functions)]
#![no_std]

extern crate rlibc;
extern crate multiboot2;

#[macro_use]
mod macros;
mod io;
mod settings;

extern { static bss: u8; }

fn main(s: &settings::Settings) {
    log!("bootloader: {:?}", s.bootloader);
    log!("cmdline:    {:?}", s.cmdline);
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

    main(&settings::Settings {
        bootloader: info.boot_loader_name_tag()
            .unwrap()
            .name(),
        cmdline: info.command_line_tag()
            .unwrap()
            .command_line()
    });

    panic!("`main` returned!");
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(fmt: core::fmt::Arguments,
        file: &'static str, line: u32) -> ! {
    log!("Fenix panicked at file: {}, line: {}; and flew away...", file,
        line);
    log!("... but he left a message before leaving:");
    log!("");
    log!("{}", fmt);

    loop {}
}
