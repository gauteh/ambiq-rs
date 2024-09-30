// The GPIO_PADKEY register must be set to 0x73 before writing to the PADREGn registers,
// and should be cleared (or set to another value) afterwards.
const PAD_KEY: u32 = 0x73;

pub(crate) fn gpio_cfg<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    cortex_m::interrupt::free(|_| {
        // page 419
        unsafe {
            (*pac::GPIO::ptr()).padkey.write(|w| w.bits(PAD_KEY));
        }

        let r = f();

        unsafe {
            (*pac::GPIO::ptr()).padkey.write(|w| w.bits(0x00));
        }

        r
    })
}

pub mod pin;
pub mod redboard_artemis;
pub mod redboard_artemis_nano;

pub use pin::Mode;
pub use pin::InterruptOpt;
pub use pin::{enable_gpio_interrupts, disable_gpio_interrupts};

#[cfg(feature = "sparkfun-redboard")]
pub use redboard_artemis::Pins;

#[cfg(feature = "sparkfun-redboard-nano")]
pub use redboard_artemis_nano::Pins;
