#![no_std]
#![no_main]
#![feature(asm)]

// pick a panicking behavior
#[cfg(not(test))]
use panic_rtt_target as _;
// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use core::borrow::BorrowMut;
use core::cell::RefCell;
use cortex_m::{asm, interrupt::Mutex};
use cortex_m_rt::entry;
use cortex_m_rt::exception;
use hal::pac::interrupt;

use rtt_target::{rprintln, rtt_init_print};

// extern crate device;

use ambiq_hal as hal;
use hal::prelude::*;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut dp = hal::pac::Peripherals::take().unwrap();
    let mut core = hal::pac::CorePeripherals::take().unwrap();

    // rprintln!("Setting VTOR to 0x10000");
    // unsafe {
    //     let SCB = &*cortex_m::peripheral::SCB::ptr();
    //     SCB.vtor.write(0x10000);
    // }

    let mut delay = hal::delay::Delay::new(core.SYST, &mut dp.CLKGEN);

    let pins = hal::gpio::Pins::new(dp.GPIO);
    let mut led = pins.d19.into_push_pull_output();

    rprintln!("Hello world from RTC example");
    rprintln!("Setting up alarm every 1 second.");

    delay.delay_ms(500u32);

    // Set up RTC
    let mut rtc = hal::rtc::Rtc::new(dp.RTC, &mut dp.CLKGEN);
    rtc.enable();

    rprintln!("RTC enabled");

    rtc.set_alarm_repeat(hal::rtc::AlarmRepeat::SEC);

    let mut timestamp = rtc.now().timestamp_millis();

    let primask = cortex_m::register::primask::read();
    rprintln!("primask active: {}", primask.is_active());

    rprintln!("interrupt::enable");
    unsafe {
        // cortex_m::asm::bkpt();
        cortex_m::interrupt::enable();
    }

    let primask = cortex_m::register::primask::read();
    rprintln!(
        "global interrupts enabled, primask: {}",
        primask.is_active()
    );

    rprintln!("RTC: enable alarm");
    rtc.enable_alarm();

    loop {
        led.toggle();

        let now = rtc.now().timestamp_millis();

        rprintln!(
            "Loop iteration, timestamp millis: {}, difference = {} (should be about 1000)",
            now,
            (now - timestamp)
        );

        timestamp = now;

        rprintln!("WFI");
        cortex_m::asm::wfi();

        delay.delay_ms(100u32);
        // cortex_m::asm::bkpt();

        // rtc.disable_alarm();
        // rtc.disable();

        let rtcnvic = hal::pac::NVIC::is_pending(hal::pac::Interrupt::RTC);
        let clknvic = hal::pac::NVIC::is_pending(hal::pac::Interrupt::CLKGEN);
        let stimnvic = hal::pac::NVIC::is_pending(hal::pac::Interrupt::STIMER);
        let other = hal::pac::NVIC::is_pending(hal::pac::Interrupt::STIMER_CMPR0);
        let rtcintstat = unsafe { (*(hal::pac::RTC::ptr())).intstat.read().alm().bit() };
        rprintln!(
            "stimnvic: {}, clknvic: {}, rtcnvic: {}, intstat: {}, other: {}",
            stimnvic,
            clknvic,
            rtcnvic,
            rtcintstat,
            other
        );
        // rtc.clear_interrupts();
        // hal::pac::NVIC::unpend(hal::pac::Interrupt::RTC);
    }
}

#[interrupt]
fn RTC() {
    // #[no_mangle]
    // pub unsafe extern "C" fn RTC() {
    // unsafe {
    // SER.as_mut().map(|serial| uwriteln!(serial, "INTERRUPT: RTC").unwrap());
    // LED.as_mut().map(|led| led.set_high());
    // }

        // cortex_m::asm::bkpt();
    cortex_m::interrupt::free(|_| {
        rprintln!("RTC interrupt");
        unsafe {
            (*(hal::pac::RTC::ptr()))
                .intclr
                .write(|w| w.alm().set_bit());
            // (*(hal::pac::RTC::ptr())).intclr.write(|w| w.bits(0x1));
        }

        // hal::pac::NVIC::unpend(hal::pac::Interrupt::RTC);
    });
}

// #[no_mangle]
// pub unsafe extern "C" fn am_default_isr() {
//     rprintln!("default interrupt");
// }
