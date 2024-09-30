#![no_std]
#![no_main]

// pick a panicking behavior
#[cfg(not(test))]
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use core::{cell::RefCell, ops::DerefMut};
use cortex_m::{
    asm,
    interrupt::{free, Mutex},
};
use cortex_m_rt::entry;

use ambiq_hal::{self as hal, gpio::Mode};
use hal::{pac::interrupt, prelude::*};

use ufmt::uwriteln;

static SERIAL: Mutex<RefCell<Option<hal::uart::Uart0<48, 49>>>> = Mutex::new(RefCell::new(None));
static A2: Mutex<RefCell<Option<hal::gpio::pin::P11<{Mode::Input}>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let mut dp = hal::pac::Peripherals::take().unwrap();
    let core = hal::pac::CorePeripherals::take().unwrap();

    let mut delay = hal::delay::Delay::new(core.SYST, &mut dp.CLKGEN);

    let pins = hal::gpio::Pins::new(dp.GPIO);
    let mut led = pins.d19.into_push_pull_output();

    // set up serial
    let mut serial = hal::uart::new_48_49(dp.UART0, pins.tx0, pins.rx0, 115000);
    uwriteln!(serial, "Hello world from GPIO example!").unwrap();

    free(|cs| {
        SERIAL.borrow(cs).replace(Some(serial));
    });

    // Set up GPIO interrupt on pin A2
    let mut a2 = pins.a2.into_input();
    a2.configure_interrupt(hal::gpio::InterruptOpt::LowToHigh);
    a2.clear_interrupt();
    a2.enable_interrupt();
    unsafe {
        hal::gpio::enable_gpio_interrupts();
        cortex_m::interrupt::enable();
    };

    free(|cs| {
        A2.borrow(cs).replace(Some(a2));
    });

    // Set up RTC
    let mut rtc = hal::rtc::Rtc::new(dp.RTC, &mut dp.CLKGEN);
    rtc.enable();

    let mut timestamp = rtc.now().timestamp_millis();
    loop {
        led.toggle().unwrap();
        delay.delay_ms(2000u32);

        let now = rtc.now().timestamp_millis();

        free(|cs| {
            let mut serial = SERIAL.borrow(cs).borrow_mut();
            let serial: &mut _ = serial.as_mut().unwrap();
            uwriteln!(
                serial,
                "Loop iteration, timestamp millis: {}, difference = {} (should be about 2000): interrupt status: {}",
                now,
                (now - timestamp),
                false
            )
            .unwrap();
        });

        // a2.set_interrupt();

        timestamp = now;
    }
}

#[allow(non_snake_case)]
#[interrupt]
fn GPIO() {
    free(|cs| {
        let mut a2 = A2.borrow(cs).borrow_mut();
        a2.as_mut().unwrap().clear_interrupt();

        let mut serial = SERIAL.borrow(cs).borrow_mut();
        let serial: &mut _ = serial.as_mut().unwrap();
        uwriteln!(serial, "GPIO interrupt!");
    });
}
