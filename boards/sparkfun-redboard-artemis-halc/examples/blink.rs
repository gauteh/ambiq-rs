#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

// pick a panicking behavior
#[cfg(not(test))]
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use ambiq_hal_sys as halc;
use halc::c_types::*;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn main(_c: *mut c_void) -> i32 {
    __main__()
}

fn __main__() -> ! {
    // Set the clock frequency.
    unsafe {
        halc::am_hal_clkgen_control(
            halc::am_hal_clkgen_control_e_AM_HAL_CLKGEN_CONTROL_SYSCLK_MAX,
            0 as *mut c_void,
        );

        // Set the default cache configuration
        halc::am_hal_cachectrl_config(&halc::am_hal_cachectrl_defaults);
        halc::am_hal_cachectrl_enable();

        // Configure the board for low power operation.
        halc::am_bsp_low_power_init();
    }

    // Set up BSP leds
    unsafe {
        let gpion = halc::am_bsp_psLEDs[0].ui32GPIONumber;
        halc::am_hal_gpio_pinconfig(gpion, halc::g_AM_HAL_GPIO_OUTPUT);
        halc::am_devices_led_off(halc::am_bsp_psLEDs.as_mut_ptr(), 0);
    }
    let mut led_state = false;

    // Blink forever
    loop {
        // Toggle LEDs
        led_state = !led_state;
        if led_state {
            unsafe {
                halc::am_devices_led_off(halc::am_bsp_psLEDs.as_mut_ptr(), 0);
            }
        } else {
            unsafe {
                halc::am_devices_led_on(halc::am_bsp_psLEDs.as_mut_ptr(), 0);
            }
        }
        // Delay
        unsafe {
            halc::am_util_delay_ms(300u32);
        }
    }
}
