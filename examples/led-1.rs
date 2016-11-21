//! Flash an LED
#![feature(lang_items)]

#![no_main]
#![no_std]

#[macro_use]
extern crate f0;

use f0::led::Color;
use f0::{delay, led};

#[export_name = "main"]
pub extern "C" fn main() -> ! {
    loop {
        Color::Blue.off();

        delay::ms(100);

        Color::Blue.on();
    }
}

// These functions are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {
}
