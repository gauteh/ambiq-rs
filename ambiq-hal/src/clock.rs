use crate::time::Hertz;

pub const CLKGEN_FREQ_MAX_HZ: Hertz = Hertz(48_000_000);
pub const CLKGEN_CLKKEY: u32 = 71;

pub use pac::CLKGEN;

#[allow(unused)]
fn clk_cfg<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    cortex_m::interrupt::free(|_| {
        unsafe {
            (*pac::CLKGEN::ptr())
                .clkkey
                .write(|w| w.bits(CLKGEN_CLKKEY));
        }

        let r = f();

        unsafe {
            (*pac::CLKGEN::ptr()).clkkey.write(|w| w.bits(0x00));
        }

        r
    })
}

pub struct ClockCtrl<'a> {
    clkgen: &'a mut CLKGEN,
}

impl<'a> ClockCtrl<'a> {
    pub fn new(clkgen: &'a mut CLKGEN) -> ClockCtrl {
        ClockCtrl { clkgen }
    }

    /// Enable the XT oscilator for RTC
    pub fn enable_xt(&mut self) {
        self.clkgen.octrl.modify(|_, w| w.stopxt().en());
    }

    /// Disable the XT oscilator for RTC
    pub fn disable_xt(&mut self) {
        self.clkgen.octrl.modify(|_, w| w.stopxt().stop());
    }

    pub fn rtc_use_xt(&mut self) {
        self.clkgen.octrl.write(|w| w.stopxt().en().osel().rtc_xt());
    }

    pub fn rtc_use_lfrc(&mut self) {
        self.clkgen.octrl.modify(|_, w| w.osel().rtc_lfrc());
    }
}
