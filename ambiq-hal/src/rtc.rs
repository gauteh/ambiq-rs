//! Real Time Clock
//!
//! TODO:
//!     * Add a Delay implementation that does deep sleep?
//!     * Arbitrary time alarm (if possible?)

use crate::clock::ClockCtrl;
use pac::{CLKGEN, RTC};
use pac::rtc::rtcctl::RPT_A;
use rtcc::DateTimeAccess;

use chrono::{Datelike, Timelike};

#[allow(unused_imports)]
use defmt::{debug, error, info, trace, warn};

pub struct Rtc {
    rtc: RTC,
}

fn bcd_to_dec(bcd: u8) -> u8 {
    (((bcd & 0xf0) >> 4) * 10) + (bcd & 0x0f)
}

fn dec_to_bcd(dec: u8) -> u8 {
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

    /// Set the current time and date (accuracy 1/100th).
    ///
    /// The century will always be the 21st (20xx).
    pub fn set(&self, dt: &chrono::NaiveDateTime) {
        let date = dt.date();
        let time = dt.time();

        let year = date.year();
        let yr = year % 100;

        debug!("set RTC to: {}-{}-{} {}:{}:{}.{}",
            year,
            date.month(),
            date.day(),
            time.hour(),
            time.minute(),
            time.second(),
            time.nanosecond());

        self.rtc.rtcctl.modify(|_, w| w.wrtc().en());

        self.rtc.ctrlow.write(|w| unsafe {
            w.ctrhr().bits(dec_to_bcd(time.hour() as u8))
                .ctrmin().bits(dec_to_bcd(time.minute() as u8))
                .ctrsec().bits(dec_to_bcd(time.second() as u8))
                .ctr100().bits(dec_to_bcd((time.nanosecond() / 1_000_000 * 100) as u8))
        });

        self.rtc.ctrup.write(|w| unsafe {
            w.ceb().dis() // TODO: support other centuries
                .ctryr()
                .bits(dec_to_bcd(yr as u8))
                .ctrmo()
                .bits(dec_to_bcd(date.month() as u8))
                .ctrdate()
                .bits(dec_to_bcd(date.day() as u8))
                .ctrwkdy()
                .bits(date.weekday() as u8)
        });

        self.rtc.rtcctl.modify(|_, w| w.wrtc().dis());
    }

    /// Get the current datetime (accurate to 1/100th second). Blocks untill no rollover
    /// error.
    pub fn now(&self) -> Option<chrono::NaiveDateTime> {
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

        let yr = bcd_to_dec(upper.ctryr().bits()) as i32;
        const CE: i32 = 20;

        chrono::NaiveDate::from_ymd_opt(
            CE * 100 + yr,
            bcd_to_dec(upper.ctrmo().bits()).into(),
            bcd_to_dec(upper.ctrdate().bits()).into(),
        ).and_then(|y| y.and_hms_milli_opt(
            bcd_to_dec(lower.ctrhr().bits()).into(),
            bcd_to_dec(lower.ctrmin().bits()).into(),
            bcd_to_dec(lower.ctrsec().bits()).into(),
            u32::from(bcd_to_dec(lower.ctr100().bits())) * 10u32,
        ))
    }

    /// Set the repeat alarm interval. Remember to enable the alarm as well.
    pub fn set_alarm_repeat(&mut self, interval: AlarmRepeat) {
        self.rtc.almup.reset();
        self.rtc.almlow.reset();

        self.rtc.rtcctl.modify(|_, w| w.rpt().variant(interval.into()));

        match interval {
            AlarmRepeat::DeciSecond => {
                self.rtc.almlow.write(|w| unsafe { w.alm100().bits(0xf0) });
            },
            AlarmRepeat::CentiSecond => {
                self.rtc.almlow.write(|w| unsafe { w.alm100().bits(0xff) });
            },
            _ => {
            }
        }
    }

    pub fn clear_interrupts(&mut self) {
        self.rtc.intclr.write(|w| w.alm().set_bit());
    }

    pub fn disable_alarm_repeat(&mut self) {
        self.rtc.rtcctl.modify(|_, w| w.rpt().dis());
    }

    pub fn enable_alarm(&mut self) {
        cortex_m::interrupt::free(|_| {
            self.clear_interrupts();
            self.rtc.inten.write(|w| w.alm().set_bit());
            unsafe {
                pac::NVIC::unmask(pac::Interrupt::RTC);
            }
        });
    }

    pub fn disable_alarm(&mut self) {
        pac::NVIC::mask(pac::Interrupt::RTC);
        self.rtc.inten.write(|w| w.alm().clear_bit());
        self.clear_interrupts();
    }
}

impl DateTimeAccess for Rtc {
    type Error = !;

    fn datetime(&mut self) -> Result<chrono::NaiveDateTime, !> {
        Ok(self.now().unwrap())
    }

    fn set_datetime(&mut self, datetime: &chrono::NaiveDateTime) -> Result<(), !> {
        self.set(datetime);

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AlarmRepeat {
    Disabled,
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,

    /// Every 100th millisecond
    DeciSecond,

    /// Every 10th millisecond
    CentiSecond,
}

impl Into<RPT_A> for AlarmRepeat {
    fn into(self) -> RPT_A {
        use AlarmRepeat::*;
        use RPT_A::*;

        match self {
            Disabled => DIS,
            Year => YEAR,
            Month => MONTH,
            Week => WEEK,
            Day => DAY,
            Hour => HR,
            Minute => MIN,
            Second => SEC,
            DeciSecond => SEC,
            CentiSecond => SEC,
        }
    }
}

