//! LEDs

use peripheral;

/// All the LEDs in the "compass" starting from "North" and going clockwise.
pub static LEDS: [Led; 2] = [Led { i: 8 },
                             Led { i: 9 }];

pub struct Led {
    i: u8,
}

impl Led {
    /// Turns the LED off
    pub fn off(&self) {
        let bsrr = &peripheral::gpioc().bsrr;
        match self.i {
            8 => bsrr.write(|w| w.br8(true)),
            9 => bsrr.write(|w| w.br9(true)),
            _ => {}
        }
    }

    /// Turns the LED on
    pub fn on(&self) {
        let bsrr = &peripheral::gpioc().bsrr;
        match self.i {
            8 => bsrr.write(|w| w.bs8(true)),
            9 => bsrr.write(|w| w.bs9(true)),
            _ => {}
        }
    }
}

/// Initializes the necessary stuff to drive the LEDs on and off
///
/// # Safety
///
/// - Must be called once
/// - Must be called in an interrupt-free environment
pub unsafe fn init() {
    let gpioc = peripheral::gpioc_mut();
    let rcc = peripheral::rcc_mut();

    // RCC: Enable GPIOC
    rcc.ahbenr.modify(|_, w| w.iopcen(true));

    // GPIOC: Configure pins 8-9 as outputs
    gpioc.moder.modify(|_, w| {
        w.moder8(0b01)
            .moder9(0b01)
    });
}

/// An enum over the LED colours
pub enum Color {
    Blue,
    Green
}

impl Color {
    pub fn on(&self) {
        match *self {
            Color::Blue => LEDS[0].on(),
            Color::Green => LEDS[1].on(),
        }
    }

    pub fn off(&self) {
        match *self {
            Color::Blue => LEDS[0].off(),
            Color::Green => LEDS[1].off(),
        }
    }
}

/// Turns off all the LEDs
pub fn all_off() {
    for led in LEDS.iter() {
        led.off();
    }
}
