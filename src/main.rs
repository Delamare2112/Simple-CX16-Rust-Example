#![no_std]
#![feature(start)]
#![allow(stable_features)]
#![feature(default_alloc_error_handler)]
#![feature(naked_functions)]
#![allow(unused_imports)]
#![feature(lang_items)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

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

#[rustfmt::skip]
static FACE: [u8; 64] = [
    1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 0, 0, 0, 0, 1, 1,
    1, 0, 1, 0, 0, 1, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 1, 0, 0, 1, 0, 1,
    1, 0, 0, 1, 1, 0, 0, 1,
    1, 1, 0, 0, 0, 0, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1];

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    println!("Hello {}!", 6502); // You'll only see this if the mode set fails.
    unsafe {
        cx16_k_screen_mode_set(128);
        println!("HELLO FROM A NEW MODE!"); // You'll only see this if console_init fails.
        cx16_k_console_init(10, 10, 320 - 20, 240 - 20);
        let message = b"\x92Hello Commander X16!";
        for c in message {
            cx16_k_console_put_char(*c, 0);
        }
        let _ = cx16_k_console_get_char();
        // Transmute to ignore *const as *mut (this function _should_ be *const)
        cx16_k_console_put_image(core::mem::transmute(FACE.as_ptr()), 8, 8);
    }
    0
}

#[cfg(not(target_vendor = "mos-cx16-none"))]
#[allow(dead_code)]
fn main() {}
