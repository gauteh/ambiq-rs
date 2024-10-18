#![no_std]
#![no_main]

// pick a panicking behavior
#[cfg(not(test))]
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

use ambiq_hal as hal;
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = hal::pac::Peripherals::take().unwrap();
    let core = hal::pac::CorePeripherals::take().unwrap();

    let mut delay = hal::delay::Delay::new(core.SYST, &mut peripherals.CLKGEN);

    let pins = hal::gpio::Pins::new(peripherals.GPIO);
    let mut led = pins.d19.into_push_pull_output();

    led.set_high().unwrap();
    delay.delay_ms(2000u32);

    led.set_low().unwrap();
    delay.delay_ms(2000u32);

    loop {
        led.toggle().unwrap();
        delay.delay_ms(100u32);
    }
}

