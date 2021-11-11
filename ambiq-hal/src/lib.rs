#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(const_fn_trait_bound)]
#![feature(never_type)]
#![cfg_attr(not(test), no_std)]

extern crate cortex_m;

pub extern crate embedded_hal as hal;
pub extern crate ambiq_apollo3_pac as pac;

#[cfg(feature = "rt")]
#[cortex_m_rt::pre_init]
unsafe fn pre_init() {
    /// The sparkfun bootloader does not update the VTOR to point to our programs
    /// interrupt vector.
    ///
    /// The Ambiq Secure bootloader requires magic bytes to be set before it loads the
    /// Sparkfun Variable Bootloader at 0xC000, so we can't just replace it.
    ///
    /// The Sparkfun bootloader loads our program from 0x10000. In the arduino hal the
    /// `startup_gcc.c` runtime updates the VTOR, but ideally this should have been done
    /// in the bootloader to be a bit more robust.
    ///
    /// If someone re-defines the `pre_init` interrupts will stop working. Pretty sure
    /// that wouldn't compile anyway.
    ///
    ///
    /// https://github.com/sparkfun/Apollo3_Uploader_SVL/issues/7
    unsafe {
        let SCB = &*cortex_m::peripheral::SCB::ptr();
        SCB.vtor.write(0x10000);
    }
}


#[cfg(feature = "ambiq-sdk")]
pub extern crate ambiq_hal_sys as halc;

pub mod clock;
pub mod rtc;
pub mod time;
pub mod delay;
pub mod gpio;

#[cfg(feature = "ambiq-sdk")]
pub mod uart;

#[cfg(feature = "ambiq-sdk")]
pub mod i2c;

pub mod prelude {
    pub use hal::prelude::*;
    pub use hal::digital::v2::{InputPin, OutputPin, ToggleableOutputPin};

    #[cfg(feature = "ambiq-sdk")]
    pub use halc;

    #[cfg(feature = "ambiq-sdk")]
    pub use halc::c_types::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
