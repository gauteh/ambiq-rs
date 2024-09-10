use crate::pac::{wdt, WDT};

pub struct Watchdog {
    wdt: WDT,
}

pub type Frequency = wdt::cfg::CLKSEL_A;

impl Watchdog {
    /// Initialize watchdog. `reset` should be true if watchdog can reset chip, the reset will
    /// occur once `reset_count` cycles of the watchdog [`Frequency`] has occurred without the
    /// watchdog being fed ([`watchdog::Watchdog::feed`]).
    pub fn from(wdt: WDT, reset: bool, reset_count: u8, freq: Frequency) -> Watchdog {
        wdt.cfg.write(|w| unsafe {
            w.resen()
                .bit(reset)
                .resval()
                .bits(reset_count)
                .clksel()
                .variant(freq)
        });

        Watchdog { wdt }
    }

    /// Start the watchdog.
    pub fn start(&mut self) {
        self.wdt.cfg.modify(|_, w| w.wdten().set_bit());
        self.feed();

        // Seems like there's a HW-bug which requires this (from C-HAL). Should just read `0`.
        self.wdt.rstrt.read().bits();
    }

    /// Locks the watchdog and starts the timer. The watchdog cannot be reconfigured.
    pub fn lock_and_start(&mut self) {
        self.wdt
            .lock
            .write(|w| w.lock().variant(wdt::lock::LOCK_A::KEYVALUE));
    }

    /// Halt the watchdog.
    pub fn halt(&mut self) {
        self.wdt.cfg.modify(|_, w| w.wdten().clear_bit());
    }

    pub fn feed(&mut self) {
        self.wdt
            .rstrt
            .write(|w| w.rstrt().variant(wdt::rstrt::RSTRT_A::KEYVALUE));
    }
}

