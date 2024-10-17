//! This example reads uart1 and pipes it to uart0 (USB)

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
// use embedded_hal::serial::Read;

use ufmt::uwriteln;

#[entry]
fn main() -> ! {
    let mut dp = hal::pac::Peripherals::take().unwrap();
    let core = hal::pac::CorePeripherals::take().unwrap();

    let mut delay = hal::delay::Delay::new(core.SYST, &mut dp.CLKGEN);

    let pins = hal::gpio::Pins::new(dp.GPIO);
    let mut led = pins.d19.into_push_pull_output();

    // set up usb-serial
    // let mut serial = hal::uart::new_48_49(dp.UART0, pins.tx0, pins.rx0, 400_000);
    // uwriteln!(serial, "Hello world from UART example!").unwrap();

    // set up uart1 on a0 and a16
    // uwriteln!(serial, "Setting up GPS serial..");
    // let mut gps_serial = hal::uart::new_12_13(dp.UART1, pins.a16, pins.a0, 400_000);
    // let uart0 = unsafe { *hal::pac::UART0::ptr() };
    let mut gps_serial = hal::uart::new_39_40(dp.UART0, pins.d9, pins.d10, 100_000);
    uwriteln!(gps_serial, "GPS serial set up.");
    use hal::uart::UartInit;
    // uwriteln!(serial, "gps serial module: {}", hal::uart::Uart1::<39, 40>::module());
    uwriteln!(gps_serial, "GPS serial set up.");

    // Set up RTC
    let mut rtc = hal::rtc::Rtc::new(dp.RTC, &mut dp.CLKGEN);
    rtc.enable();

    let mut timestamp = rtc.now().timestamp_millis();
    loop {
        led.toggle().unwrap();
        uwriteln!(gps_serial, "GPS serial set up.");

        match gps_serial.read() {
            Ok(w) => {
                // serial.write(w).ok();
                gps_serial.write(w).ok();
            },
            Err(nb::Error::WouldBlock) => { /* wait */ } // TODO: timeout!
            Err(nb::Error::Other(e)) => {
                // uwriteln!(serial, "ext-gps: error reading from uart: {:?}", e);
            }
        }
    }
}

