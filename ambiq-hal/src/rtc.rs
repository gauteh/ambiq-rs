use pac::{RTC, CLKGEN};

use crate::clock::ClockCtrl;

pub struct Rtc {
    rtc: RTC,
}

impl Rtc {
    pub fn new(rtc: RTC, clkgen: &mut CLKGEN) -> Rtc {
        // Enable XT for RTC
        let mut clk = ClockCtrl::new(clkgen);
        clk.enable_xt();

        // Select XT as source
        clk.rtc_use_xt();

        Rtc { rtc }
    }

    pub fn enable(&mut self) {
        self.rtc.rtcctl.modify(|_, w| w.rstop().run());
    }

    pub fn disable(&mut self) {
        self.rtc.rtcctl.modify(|_, w| w.rstop().stop());
    }
}
