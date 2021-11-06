#![no_std]

pub use cortex_m_rt::entry;
pub use ambiq_hal as hal;
pub use hal::prelude;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
