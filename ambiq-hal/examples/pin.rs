#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

extern crate ambiq_hal as hal;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let perip = hal::pac::Peripherals::take().unwrap();

    let pins1 = hal::gpio::Pins::new(perip.GPIO);
    // let pins2 = hal::gpio::Pins::new(perip.GPIO);

    let adc = perip.ADC;

    loop {}
}
