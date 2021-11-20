#![no_std]
#![no_main]
#![allow(non_snake_case)]

use panic_halt as _;

pub mod flashdev;

use ambiq_hal as hal;
use cortex_m_rt::entry;
use cortex_m;

#[entry]
fn main() -> ! {
    // println!("Hello, world!");

    loop {
        cortex_m::asm::nop();
    }
}
