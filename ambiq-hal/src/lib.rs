#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(const_fn_trait_bound)]
#![feature(never_type)]
#![cfg_attr(not(test), no_std)]

extern crate cortex_m;

pub extern crate embedded_hal as hal;
pub extern crate ambiq_apollo3_pac as pac;


#[cfg(feature = "ambiq-sdk")]
pub extern crate ambiq_hal_sys as halc;

pub mod clock;
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
