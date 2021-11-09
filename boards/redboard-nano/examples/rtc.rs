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

use ufmt::uwriteln;

#[entry]
fn main() -> ! {
    let mut dp = hal::pac::Peripherals::take().unwrap();
    let core = hal::pac::CorePeripherals::take().unwrap();

    let mut delay = hal::delay::Delay::new(core.SYST, &mut dp.CLKGEN);

    let pins = hal::gpio::Pins::new(dp.GPIO);
    let mut led = pins.d19.into_push_pull_output();

    // set up serial
    let mut serial = hal::uart::Uart0::new(dp.UART0, pins.tx0, pins.rx0);
    uwriteln!(serial, "Hello world from RTC example!").unwrap();

    // Set up RTC
    let mut rtc = hal::rtc::Rtc::new(dp.RTC, &mut dp.CLKGEN);
    rtc.enable();

    let mut timestamp = rtc.now().timestamp_millis();
    loop {
        led.toggle().unwrap();
        delay.delay_ms(2000u32);

        let now = rtc.now().timestamp_millis();

        uwriteln!(serial, "Loop iteration, timestamp millis: {}, difference = {} (should be about 2000)", now, (now - timestamp)).unwrap();

        timestamp = now;
    }
}

