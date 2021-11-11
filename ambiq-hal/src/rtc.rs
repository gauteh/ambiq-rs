use crate::clock::ClockCtrl;
use embedded_time::Clock;
use pac::{CLKGEN, RTC};

pub struct Rtc {
    rtc: RTC,
}

pub fn bcd_to_dec(bcd: u8) -> u8 {
    (((bcd & 0xf0) >> 4) * 10) + (bcd & 0x0f)
}

pub fn dec_to_bcd(dec: u8) -> u8 {
    ((dec / 10) << 4) | (dec % 10)
}

impl Rtc {
    pub fn new(rtc: RTC, clkgen: &mut CLKGEN) -> Rtc {
        // Enable XT for RTC
        let mut clk = ClockCtrl::new(clkgen);
        // clk.enable_xt();
        rtc.rtcctl.reset();
        rtc.almup.reset();
        rtc.almlow.reset();
        rtc.inten.write(|w| w.alm().clear_bit());
        rtc.intclr.write(|w| w.alm().set_bit());

        // Select XT as source
        clk.rtc_use_xt();

        rtc.rtcctl.modify(|_, w| w.hr1224()._24hr());

        Rtc { rtc }
    }

    pub fn enable(&mut self) {
        self.rtc.rtcctl.modify(|_, w| w.rstop().run());
    }

    pub fn disable(&mut self) {
        self.rtc.rtcctl.modify(|_, w| w.rstop().stop());
    }

    /// Get the current datetime (accurate to 1/100th second). Blocks untill no rollover
    /// error.
    pub fn now(&self) -> chrono::NaiveDateTime {
        let (upper, lower) = loop {
            let lower = self.rtc.ctrlow.read();

            let no_err = self.rtc.ctrup.read().cterr().is_noerr(); // Set if upper read is done later than 1/100th sec after lower read.

            let upper = self.rtc.ctrup.read(); // Resets error.

            // Check for rollover between read of lower and upper.
            // p. 554.
            if no_err {
                break (upper, lower);
            }
        };

        chrono::NaiveDate::from_ymd(
            bcd_to_dec(upper.ctryr().bits()).into(),
            bcd_to_dec(upper.ctrmo().bits()).into(),
            bcd_to_dec(upper.ctrdate().bits()).into(),
        )
        .and_hms_milli(
            bcd_to_dec(lower.ctrhr().bits()).into(),
            bcd_to_dec(lower.ctrmin().bits()).into(),
            bcd_to_dec(lower.ctrsec().bits()).into(),
            u32::from(bcd_to_dec(lower.ctr100().bits())) * 10u32,
        )
    }

    /// Set the repeat alarm interval. Remember to enable the alarm as well.
    pub fn set_alarm_repeat(&mut self, interval: AlarmRepeat) {
        // TODO: Also support 1/10th and 1/100th second alarms.

        unsafe {
            // self.rtc.almup.write(|w| w.bits(0x0));
            // self.rtc.almlow.write(|w| w.bits(0x0));
            self.rtc.almup.reset();
            self.rtc.almlow.reset();
            // self.rtc.almlow.write(|w| w.alm100().bits(0));
            // self.rtc.almlow.write(|w| w.alm100().bits(0));
        }
        self.rtc.rtcctl.modify(|_, w| w.rpt().variant(interval));
    }

    pub fn clear_interrupts(&mut self) {
        unsafe {
            // self.rtc.intclr.write(|w| w.bits(0x8));
            self.rtc.intclr.write(|w| w.alm().set_bit());
        }
    }

    pub fn disable_alarm_repeat(&mut self) {
        self.rtc.rtcctl.modify(|_, w| w.rpt().dis());
    }

    pub fn enable_alarm(&mut self) {
        // cortex_m::interrupt::free(|_| {
            self.clear_interrupts();
            self.rtc.inten.write(|w| w.alm().set_bit());
            unsafe {
                pac::NVIC::unmask(pac::Interrupt::RTC);
            }
        // });
    }

    pub fn disable_alarm(&mut self) {
        self.rtc.inten.write(|w| w.alm().clear_bit());
        unsafe {
            pac::NVIC::mask(pac::Interrupt::RTC);
        }
        self.clear_interrupts();
    }
}

pub type AlarmRepeat = pac::rtc::rtcctl::RPT_A;

// impl Clock for Rtc {
//     type T = u64;
//     const SCALING_FACTOR =
// }
