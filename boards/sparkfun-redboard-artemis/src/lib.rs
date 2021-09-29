#![no_std]

pub use cortex_m_rt::entry;
pub use ambiq_hal as hal;
pub use hal::prelude;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
