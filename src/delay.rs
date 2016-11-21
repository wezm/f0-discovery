//! Delays
//!
//! # Implementation details
//!
//! This module uses the TIM6 peripheral and the `_tim6` interrupt under the
//! hood.

use peripheral;

/// Blocks for `n` ms
pub fn ms(n: u16) {
    unsafe {
        let tim6 = peripheral::tim6_mut();

        // The alarm (the "update event") will set off in `n` "ticks".
        // One tick = 1 ms (see `init`)
        tim6.arr.write(|w| w.arr(n));

        // Generate an update event to update the autoreload register
        tim6.egr.write(|w| w.ug(true));

        // Clear any previous "update" event by clearing the update event flag
        tim6.sr.read();
        tim6.sr.write(|w| w);

        // CEN: Enable the counter
        tim6.cr1.modify(|_, w| w.cen(true));

        // Wait until the alarm goes off (the "update event" occurs)
        while !tim6.sr.read().uif() {}

        // Clear the "update" flag
        tim6.sr.write(|w| w);
    }
}

/// Initializes the necessary stuff to be able to use delays
///
/// # Safety
///
/// - Must be called once
/// - Must be called in an interrupt-free environment
pub unsafe fn init() {
    let rcc = peripheral::rcc_mut();
    let tim6 = peripheral::tim6_mut();

    // RCC: Enable TIM6
    rcc.apb1enr.modify(|_, w| w.tim6en(true));

    // CEN: Disable the clock
    // OPM. Enable One Pulse Mode. Stop the counter after the next update event.
    tim6.cr1.write(|w| w.cen(false).opm(true));

    // Set pre-scaler to 8_000 -> Frequency = 1 KHz
    tim6.psc.write(|w| w.psc(7_999));
}
