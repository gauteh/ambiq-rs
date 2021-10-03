#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(const_fn_trait_bound)]
#![no_std]

pub extern crate embedded_hal as hal;
pub extern crate ambiq_apollo3_pac as pac;

pub mod clock;
pub mod time;
pub mod delay;
pub mod gpio;

pub mod prelude {
    pub use hal::prelude::*;

    pub use hal::digital::v2::{InputPin, OutputPin, ToggleableOutputPin};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
