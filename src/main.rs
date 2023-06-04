#![no_std]
#![feature(start)]
#![allow(stable_features)]
#![feature(default_alloc_error_handler)]
#![allow(unused_imports)]
#![feature(lang_items)]

use core::panic::PanicInfo;
use ufmt_stdio::*;

#[cfg(not(target_vendor = "mos-cx16-none"))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("PANIC!!!");
    loop {}
}

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    println!("Hello {}!", 6502);
    0
}

#[cfg(not(target_vendor = "mos-cx16-none"))]
#[allow(dead_code)]
fn main() {}
