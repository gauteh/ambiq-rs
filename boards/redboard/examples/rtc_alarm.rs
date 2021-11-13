#![no_std]
#![no_main]

// pick a panicking behavior
use panic_rtt_target as _;

use cortex_m_rt::entry;
use hal::pac::interrupt;

use rtt_target::{rprintln, rtt_init_print};

use ambiq_hal as hal;
use hal::prelude::*;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut dp = hal::pac::Peripherals::take().unwrap();
    let mut core = hal::pac::CorePeripherals::take().unwrap();

    let mut delay = hal::delay::Delay::new(core.SYST, &mut dp.CLKGEN);

    let pins = hal::gpio::Pins::new(dp.GPIO);
    let mut led = pins.d13.into_push_pull_output();

    rprintln!("Hello world from RTC example");
    rprintln!("Setting up alarm every 1 second.");

    // Set up RTC
    let mut rtc = hal::rtc::Rtc::new(dp.RTC, &mut dp.CLKGEN);
    rtc.enable();
    rtc.set_alarm_repeat(hal::rtc::AlarmRepeat::SEC);

    rprintln!("RTC: enable alarm");
    rtc.enable_alarm();

    rprintln!("Enable interrupts");
    unsafe {
        cortex_m::interrupt::enable();
    }

    let mut timestamp = rtc.now().timestamp_millis();

    loop {
        led.toggle();

        rprintln!("Waiting for interrupt (sleeping)..");
        cortex_m::asm::wfi();

        let now = rtc.now().timestamp_millis();

        rprintln!(
            "Loop iteration, timestamp millis: {}, difference = {} (should be about 1000)",
            now,
            (now - timestamp)
        );

        timestamp = now;
    }
}

#[interrupt]
fn RTC() {
    cortex_m::interrupt::free(|_| {
        rprintln!("RTC interrupt");

        // Clear RTC interrupt
        unsafe {
            (*(hal::pac::RTC::ptr()))
                .intclr
                .write(|w| w.alm().set_bit());
        }
    });
}
