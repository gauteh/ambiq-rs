#![no_std]
#![no_main]
#![feature(asm)]

// pick a panicking behavior
#[cfg(not(test))]
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use core::borrow::BorrowMut;
use core::cell::RefCell;
use cortex_m::{asm, interrupt::Mutex};
use cortex_m_rt::entry;
use hal::pac::interrupt;
use cortex_m_rt::exception;

// extern crate device;

use ambiq_hal as hal;
use hal::prelude::*;

use ufmt::uwriteln;

static mut SER: Option<hal::uart::Uart0> = None;
static mut LED: Option<hal::gpio::pin::P19<{ hal::gpio::pin::Mode::Output }>> = None;

#[entry]
fn main() -> ! {
    let mut dp = hal::pac::Peripherals::take().unwrap();
    let mut core = hal::pac::CorePeripherals::take().unwrap();

    let mut delay = hal::delay::Delay::new(core.SYST, &mut dp.CLKGEN);

    let pins = hal::gpio::Pins::new(dp.GPIO);
    let mut led = pins.d19.into_push_pull_output();

    // set up serial
    let mut serial = hal::uart::Uart0::new(dp.UART0, pins.tx0, pins.rx0);
    uwriteln!(serial, "Hello world from RTC example!").unwrap();
    uwriteln!(serial, "Setting up alarm every 1 second.").unwrap();

    delay.delay_ms(500u32);

    // Set up RTC
    let mut rtc = hal::rtc::Rtc::new(dp.RTC, &mut dp.CLKGEN);
    delay.delay_ms(500u32);
    rtc.enable();
    let mut timestamp = rtc.now().timestamp_millis();

    uwriteln!(serial, "rtc enabled").unwrap();

    delay.delay_ms(500u32);
    rtc.set_alarm_repeat(hal::rtc::AlarmRepeat::SEC);
    delay.delay_ms(500u32);
    // rtc.enable_alarm();

    timestamp = rtc.now().timestamp_millis();


    let primask = cortex_m::register::primask::read();
    uwriteln!(serial, "primask active: {}", primask.is_active());


    uwriteln!(serial, "enabling global interrupts").unwrap();

    unsafe {
        SER = Some(serial);
        LED = Some(led);
    }

    rtc.enable_alarm();
    unsafe {
        SER.as_mut().map(|serial| uwriteln!(serial, "rtc enabled alarm").unwrap());
    }

    unsafe {
        cortex_m::interrupt::enable();
    }

    let primask = cortex_m::register::primask::read();
    unsafe {
        SER.as_mut().map(|serial| uwriteln!(serial, "global interrupts enabled, primask: {}", primask.is_active()).unwrap());
    }



    // unsafe {
    //     halc::am_hal_interrupt_master_enable();
    // }


    loop {
        unsafe {
            LED.as_mut().map(|led| led.toggle());
        }

        let now = rtc.now().timestamp_millis();

        unsafe {
            SER.as_mut().map(|ser| {
                uwriteln!(
                    ser,
                    "Loop iteration, timestamp millis: {}, difference = {} (should be about 1000)",
                    now,
                    (now - timestamp)
                )
                .unwrap();
            });
        }

        timestamp = now;

        unsafe {
            SER.as_mut().map(|serial| uwriteln!(serial, "Sleeping (WFI)..").unwrap());
        }

        cortex_m::asm::wfi();

        delay.delay_ms(100u32);
        // cortex_m::asm::bkpt();

        // rtc.disable_alarm();
        // rtc.disable();

        let rtcnvic = hal::pac::NVIC::is_pending(hal::pac::Interrupt::RTC);
        let clknvic = hal::pac::NVIC::is_pending(hal::pac::Interrupt::CLKGEN);
        let stimnvic = hal::pac::NVIC::is_pending(hal::pac::Interrupt::STIMER);
        let rtcintstat = unsafe { (*(hal::pac::RTC::ptr())).intstat.read().alm().bit() };
        unsafe {
            SER.as_mut().map(|ser| {
                uwriteln!(ser, "stimnvic: {}, clknvic: {}, rtcnvic: {}, intstat: {}", stimnvic, clknvic, rtcnvic, rtcintstat).unwrap();
            });
        }
        // rtc.clear_interrupts();
        // hal::pac::NVIC::unpend(hal::pac::Interrupt::RTC);
    }
}

#[interrupt]
fn RTC() {
// #[no_mangle]
// pub unsafe extern "C" fn RTC() {
    unsafe {
        // SER.as_mut().map(|serial| uwriteln!(serial, "INTERRUPT: RTC").unwrap());
        LED.as_mut().map(|led| led.set_high());
    }

    // cortex_m::asm::bkpt();

    cortex_m::interrupt::free(|_| {
        unsafe {
            (*(hal::pac::RTC::ptr())).intclr.write(|w| w.alm().set_bit());
            // (*(hal::pac::RTC::ptr())).intclr.write(|w| w.bits(0x1));
        }

        hal::pac::NVIC::unpend(hal::pac::Interrupt::RTC);
    });
}

#[interrupt]
fn STIMER() {
    unsafe {
        LED.as_mut().map(|led| led.set_high());
    }
    hal::pac::NVIC::unpend(hal::pac::Interrupt::STIMER);
    hal::pac::NVIC::mask(hal::pac::Interrupt::STIMER);
}

#[interrupt]
fn GPIO() {
    unsafe {
        LED.as_mut().map(|led| led.set_high());
    }
    hal::pac::NVIC::unpend(hal::pac::Interrupt::GPIO);
    hal::pac::NVIC::mask(hal::pac::Interrupt::GPIO);
}

#[interrupt]
fn UART0() {
    unsafe {
        LED.as_mut().map(|led| led.set_high());
    }
    hal::pac::NVIC::unpend(hal::pac::Interrupt::UART0);
    hal::pac::NVIC::mask(hal::pac::Interrupt::UART0);
}

#[interrupt]
fn UART1() {
    unsafe {
        LED.as_mut().map(|led| led.set_high());
    }
    hal::pac::NVIC::unpend(hal::pac::Interrupt::UART1);
    hal::pac::NVIC::mask(hal::pac::Interrupt::UART1);
}

#[interrupt]
fn IOSLAVE() {
    unsafe {
        LED.as_mut().map(|led| led.set_high());
    }
    hal::pac::NVIC::unpend(hal::pac::Interrupt::IOSLAVE);
    hal::pac::NVIC::mask(hal::pac::Interrupt::IOSLAVE);
}

#[interrupt]
fn IOSLAVEACC() {
    unsafe {
        LED.as_mut().map(|led| led.set_high());
    }
    hal::pac::NVIC::unpend(hal::pac::Interrupt::IOSLAVEACC);
    hal::pac::NVIC::mask(hal::pac::Interrupt::IOSLAVEACC);
}

#[interrupt]
fn CLKGEN() {
    unsafe {
        LED.as_mut().map(|led| led.set_high());
    }
}

#[exception]
unsafe fn DefaultHandler(i: i16) -> ! {
    loop {
        unsafe {
            LED.as_mut().map(|led| led.set_high());
        }
    }
}

#[no_mangle]
pub extern "C" fn am_default_isr() {
    unsafe {
        LED.as_mut().map(|led| led.set_high());
    }
    // unsafe {
    //     // SER.as_mut().map(|serial| uwriteln!(serial, "INTERRUPT: RTC (HALC)").unwrap());
    //     LED.as_mut().map(|led| led.set_high());
    // }
    unsafe {
        (*(hal::pac::RTC::ptr())).intclr.write(|w| w.alm().set_bit());
    }
    // hal::pac::NVIC::unpend(hal::pac::Interrupt::RTC);
}
