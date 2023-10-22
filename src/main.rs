#![no_std]
#![no_main]

use core::{
    arch::{asm, global_asm},
    ffi::c_void,
};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern "C" {
    static __bss: c_void;
    static __bss_end: c_void;
    static __stack_top: c_void;
}

global_asm!(
    "
.section .text.init
.global start
start:
    la sp, __stack_top
    j os_main
    "
);

fn sbi_putchar(ch: u8) {
    unsafe {
        asm!(
            "ecall",
            in("a7") 1,
            in("a0") ch as usize,
            lateout("a0")_
        );
    }
}

#[no_mangle]
fn os_main(_hartid: usize, _dtb: usize) -> ! {
    unsafe {
        let bss = &__bss as *const _ as *mut u8;
        let bss_end = &__bss_end as *const _;
        bss.write_bytes(0, bss_end as usize - bss as usize);
    };

    for ch in "Hello, World!\n".bytes() {
        sbi_putchar(ch);
    }

    loop {}
}
