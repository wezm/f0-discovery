//! Override `panic_fmt`

#![feature(asm)]
#![no_main]
#![no_std]

#[macro_use]
extern crate f3;

use core::fmt::Arguments;

#[export_name = "main"]
pub extern "C" fn main() -> ! {
    // Panic here and ...
    panic!("Hello, world!")
}

#[allow(dead_code)]
#[export_name = "rust_begin_unwind"]
extern "C" fn panic_fmt(_msg: Arguments, _file: &'static str, _line: u32) -> ! {
    unsafe {
        // ... you'll reach this breakpoint!
        bkpt!();
    }

    loop {}
}
