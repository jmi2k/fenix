pub unsafe fn out_u8(port: u16, val: u8) {
    asm!("outb %al, %dx"
        :: "{al}"(val), "{dx}"(port));
}

pub unsafe fn in_u8(port: u16) -> u8 {
    let val: u8;

    asm!("inb %dx, %al"
        : "={al}"(val)
        : "{dx}"(port));
    val
}

pub unsafe fn out_u16(port: u16, val: u16) {
    asm!("outw %ax, %dx"
        :: "{ax}"(val), "{dx}"(port));
}

pub unsafe fn in_u16(port: u16) -> u16 {
    let val: u16;

    asm!("inw %dx, %ax"
        : "={ax}"(val)
        : "{dx}"(port));
    val
}

pub unsafe fn out_u32(port: u16, val: u32) {
    asm!("outl %eax, %dx"
        :: "{eax}"(val), "{dx}"(port));
}

pub unsafe fn in_u32(port: u16) -> u32 {
    let val: u32;

    asm!("inl %dx, %eax"
        : "={eax}"(val)
        : "{dx}"(port));
    val
}
