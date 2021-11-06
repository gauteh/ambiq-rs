use crate::clock::ClockCtrl;
use embedded_time::Clock;
use pac::{CLKGEN, RTC};

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

    /// Get the current datetime (accurate to 100th second). Blocks untill no rollover
    /// error.
    pub fn now(&self) -> chrono::NaiveDateTime {
        let (upper, lower) = loop {
            let lower = self.rtc.ctrlow.read();
            let upper = self.rtc.ctrup.read();

            // Check whether rollover between read of lower or upper.
            // p. 554.
            if !self.rtc.ctrup.read().cterr().is_rderr() {
                break (upper, lower);
            }
        };

        chrono::NaiveDate::from_ymd(
            upper.ctryr().bits().into(),
            upper.ctrmo().bits().into(),
            upper.ctrdate().bits().into(),
        )
        .and_hms_milli(
            lower.ctrhr().bits().into(),
            lower.ctrmin().bits().into(),
            lower.ctrsec().bits().into(),
            u32::from(lower.ctr100().bits()) * 10u32,
        )
    }
}

// impl Clock for Rtc {
//     type T = u64;
//     const SCALING_FACTOR =
// }
