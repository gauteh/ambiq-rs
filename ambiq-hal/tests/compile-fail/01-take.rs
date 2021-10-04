#![no_std]
#![no_main]

extern crate panic_halt;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate ambiq_hal;

use ambiq_hal as hal;
use hal::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    let perip = hal::pac::Peripherals::take().unwrap();

    let pins1 = hal::gpio::Pins::new(perip.GPIO);

    // // This causes compile error:
    let pins2 = hal::gpio::Pins::new(perip.GPIO);

    // let _adc = perip.ADC;

    // // Set up led as GPIO output pin.
    // let mut led = pins1.d13.into_push_pull_output();

    // loop {
    //     led.toggle().unwrap();
    // }
    //
    loop { }
}

