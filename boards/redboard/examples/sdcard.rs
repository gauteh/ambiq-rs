//! To receive RTT output using JLink

#![no_std]
#![no_main]

// pick a panicking behavior
#[cfg(not(test))]
use panic_probe as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use defmt_rtt as _;

use cortex_m_rt::entry;
use cortex_m;
use defmt::{println, info, debug, warn, error, trace};

use embedded_hal::spi;
use embedded_sdmmc::{SdMmcSpi};
use ambiq_hal as hal;
use hal::prelude::*;


#[entry]
fn main() -> ! {
    let mut dp = hal::pac::Peripherals::take().unwrap();
    let core = hal::pac::CorePeripherals::take().unwrap();

    let mut delay = hal::delay::Delay::new(core.SYST, &mut dp.CLKGEN);

    // SPI0 shares SCK with the led on d13

    let pins = hal::gpio::Pins::new(dp.GPIO);

    println!("Setting up SPI..");
    let spi = hal::spi::Spi0::new(dp.IOM0, pins.d11, pins.d12, pins.d13, hal::spi::Freq::F4mHz, spi::MODE_0);
    let cs = pins.d10.into_push_pull_output();

    println!("Setting up SD card..");
    let mut sd = SdMmcSpi::new(spi, cs);
    let sd = sd.acquire().unwrap();

    println!("Card size: {}", sd.card_size_bytes().unwrap());


    loop {
        // led.toggle().unwrap();
        delay.delay_ms(1000u32);
        println!("Hello, world!");
    }
}

