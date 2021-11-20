//! To receive RTT output using JLink

#![no_std]
#![no_main]

// pick a panicking behavior
#[cfg(not(test))]
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use cortex_m;

use ambiq_hal as hal;
use hal::prelude::*;

use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut peripherals = hal::pac::Peripherals::take().unwrap();
    let core = hal::pac::CorePeripherals::take().unwrap();

    let mut delay = hal::delay::Delay::new(core.SYST, &mut peripherals.CLKGEN);

    let pins = hal::gpio::Pins::new(peripherals.GPIO);
    let mut led = pins.d19.into_push_pull_output();

    loop {
        led.toggle().unwrap();
        delay.delay_ms(1000u32);
        rprintln!("Hello, world!");
    }
}

