//! Delays
//!
//! TODO:
//!
//!     * Use cortex-m's Delay
//!     * Add a busy-wait function?

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;

use crate::clock;
use crate::time::Hertz;
use hal::blocking::delay::{DelayMs, DelayUs};
use pac::CLKGEN;

/// System timer (SysTick) as a delay provider
pub struct Delay {
    sysclock: Hertz,
    syst: SYST,
}

impl Delay {
    /// Configures the system timer (SysTick) as a delay provider
    pub fn new(mut syst: SYST, clkgen: &mut CLKGEN) -> Self {
        syst.set_clock_source(SystClkSource::Core);

        // figure out clock frequency of system clock
        let sysclock = match clkgen.cctrl.read().coresel().variant() {
            pac::clkgen::cctrl::CORESEL_A::HFRC => {
                // full frequency
                clock::CLKGEN_FREQ_MAX_HZ
            }
            pac::clkgen::cctrl::CORESEL_A::HFRC_DIV2 => {
                // half
                Hertz(clock::CLKGEN_FREQ_MAX_HZ.0 / 2)
            }
        };

        Delay {
            syst,
            sysclock: sysclock.into(),
        }
    }

    /// Releases the system timer (SysTick) resource
    pub fn free(self) -> SYST {
        self.syst
    }
}

impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms * 1_000);
    }
}

impl DelayMs<u16> for Delay {
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(ms as u32);
    }
}

impl DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(ms as u32);
    }
}

impl DelayUs<u32> for Delay {
    fn delay_us(&mut self, us: u32) {
        // The SysTick Reload Value register supports values between 1 and 0x00FFFFFF.
        const MAX_RVR: u32 = 0x00FF_FFFF;

        let mut total_rvr = us * (self.sysclock.0 / 1_000_000);

        while total_rvr != 0 {
            let current_rvr = if total_rvr <= MAX_RVR {
                total_rvr
            } else {
                MAX_RVR
            };

            self.syst.set_reload(current_rvr);
            self.syst.clear_current();
            self.syst.enable_counter();

            // Update the tracking variable while we are waiting...
            total_rvr -= current_rvr;

            while !self.syst.has_wrapped() {}

            self.syst.disable_counter();
        }
    }
}

impl DelayUs<u16> for Delay {
    fn delay_us(&mut self, us: u16) {
        self.delay_us(us as u32)
    }
}

impl DelayUs<u8> for Delay {
    fn delay_us(&mut self, us: u8) {
        self.delay_us(us as u32)
    }
}


#[cfg(feature = "ambiq-sdk")]
pub use flash::FlashDelay;

#[cfg(feature = "ambiq-sdk")]
pub mod flash {
    use super::*;

    /// Uses the bootrom to implement a spin loop. Use this struct to busy wait in a spin loop without cache
    /// or delay uncertainties.
    ///
    /// Notes for Apollo3:
    /// - The ROM-based function executes at 3 cycles per iteration plus the normal
    ///   function call, entry, and exit overhead and latencies.
    /// - Cache settings affect call overhead.  However, the cache does not affect
    ///   the time while inside the BOOTROM function.
    /// - The function accounts for burst vs normal mode, along with some of the
    ///   overhead encountered with executing the function itself (such as the
    ///   check for burst mode).
    /// - Use of the FLASH_CYCLES_US() or FLASH_CYCLES_US_NOCACHE() macros for the
    ///   ui32Iterations parameter will result in approximate microsecond timing.
    /// - The parameter us==0 is allowed but is still incurs a delay.
    ///
    /// > Interrupts are not disabled during execution of this function.
    /// > Therefore, any interrupt taken will affect the delay timing.
    #[derive(Clone, Copy)]
    pub struct FlashDelay;

    impl FlashDelay {
        pub fn new() -> FlashDelay {
            FlashDelay
        }
    }

    impl DelayUs<u32> for FlashDelay {
        fn delay_us(&mut self, us: u32) {
            // Get clock frequency.
            let clkgen = unsafe { &*CLKGEN::ptr() };
            let sysclock = match clkgen.cctrl.read().coresel().variant() {
                pac::clkgen::cctrl::CORESEL_A::HFRC => {
                    // full frequency
                    clock::CLKGEN_FREQ_MAX_HZ
                }
                pac::clkgen::cctrl::CORESEL_A::HFRC_DIV2 => {
                    // half
                    Hertz(clock::CLKGEN_FREQ_MAX_HZ.0 / 2)
                }
            };

            let cycles = us * (sysclock.0 / 3_000_000);

            unsafe {
                halc::am_hal_flash_delay(cycles);
            }
        }
    }

    impl DelayUs<u16> for FlashDelay {
        fn delay_us(&mut self, us: u16) {
            self.delay_us(us as u32)
        }
    }

    impl DelayUs<u8> for FlashDelay {
        fn delay_us(&mut self, us: u8) {
            self.delay_us(us as u32)
        }
    }

    impl<T> DelayMs<T> for FlashDelay where T: Into<u32> {
        fn delay_ms(&mut self, us: T) {
            let us = us.into();
            DelayUs::<u32>::delay_us(self, us * 1000);
        }
    }
}
