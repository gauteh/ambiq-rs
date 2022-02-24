#[allow(unused_imports)]
use defmt::{debug, error, info, trace, warn};
use crate::{pac, delay::FlashDelay, clock};

pub mod i2c;
pub mod spi;

#[derive(defmt::Format)]
enum Direction {
    Write = 0x00,
    Read = 0x01,
}

#[derive(ufmt::derive::uDebug, Debug, Copy, Clone, defmt::Format)]
pub enum IomError {
    /// This indicates an error outside of our control, no such device, etc.
    WriteHwError,

    /// Arbitrier (?) error.
    ARB,

    /// NAK received.
    NAK,

    Other,

    /// This indicates that the HW interface was operated wrongly, usually by an error in this
    /// driver.
    SwError,

    /// Have to read more than zero bytes.
    ReadTooFew,

    Timeout,
}

trait IomFreq {
    fn freq(&self) -> u32;
}

/// Common functionality between the I2C and SPI modules.
trait Iom {
    fn is_ready(&self) -> bool;

    fn wait_transfer(&self) -> Result<(), IomError> {
        defmt::trace!("wait transfer..");
        // wait for previous transfer, this check is only necessary if
        // the previous write was aborted due to e.g. a timeout.
        for _ in 0..500_000 {
            if self.is_ready() {
                defmt::trace!("wait transfer: transfer done.");
                return Ok(());
            }

            cortex_m::asm::nop();
        }

        defmt::warn!("wait transfer: timed out!");
        Err(IomError::Timeout)
    }

    /// Resets the I2C module and clears FIFOs.
    fn reset(&self);

    fn clear_interrupts(&self);
    fn disable_interrupts(&self) -> u32;
    fn enable_interrupts(&self, inten: u32);

    fn check_error(&self) -> Result<(), IomError>;

    fn push_fifo(&self, word: &[u8]);
    fn pop_fifo(&self, buffer: &mut [u8]);
}

impl Iom for pac::iom0::RegisterBlock {
    fn check_error(&self) -> Result<(), IomError> {
        match self.intstat.read() {
            i if i.icmd().bit() || i.fovfl().bit() || i.fundfl().bit() || i.iacc().bit() => {
                Err(IomError::SwError)
            }
            i if i.arb().bit() || i.start().bit() || i.stop().bit() => Err(IomError::ARB),
            i if i.nak().bit() => Err(IomError::NAK),
            i if i.cqerr().bit() || i.derr().bit() => Err(IomError::Other),
            _ => Ok(()),
        }
    }

    fn is_ready(&self) -> bool {
        let status = self.status.read();
        status.idlest().is_idle() && !status.cmdact().is_active()
    }

    fn reset(&self) {
        defmt::warn!("i2c: resetting module.");

        let inten = self.disable_interrupts();

        self.wait_transfer().ok(); // XXX: Ignoring this, maybe better to reset the module if it
                                   // doesn't work?.

        // disable the submodule
        self.submodctrl.modify(|_r, w| w.smod1en().clear_bit());

        // reset FIFO
        self.fifoctrl.modify(|_r, w| w.fiforstn().clear_bit());

        defmt::trace!("i2c: reset: waiting for submodule");
        // delay for "> 6 clocks"?
        let wait = clock::CLKGEN_FREQ_MAX_HZ.0 / 2 / 100_000; // longest possible delay (for i2c)
        FlashDelay::delay_cycles(wait);
        defmt::trace!("i2c: reset: waiting for submodule: done");

        self.fifoctrl.modify(|_r, w| w.fiforstn().set_bit());
        self.submodctrl.modify(|_r, w| w.smod1en().set_bit());

        self.clear_interrupts();
        self.enable_interrupts(inten);
    }

    fn clear_interrupts(&self) {
        unsafe {
            self.intclr.write(|i| i.bits(0xFFFF_FFFF));
        }
    }

    fn disable_interrupts(&self) -> u32 {
        let inten = self.inten.read().bits();

        unsafe {
            // Disable IOM interrupts
            self.inten.write(|i| i.bits(0));

            // Disable DMA. We are doing direct writes / reads with busy polling.
            self.dmacfg.modify(|_, dw| dw.dmaen().dis());
        }

        inten
    }

    fn enable_interrupts(&self, inten: u32) {
        unsafe {
            self.inten.write(|i| i.bits(inten));
        }
    }

    fn push_fifo(&self, word: &[u8]) {
        let word = if word.len() == 4 {
            u32::from_ne_bytes(word.try_into().unwrap())
        } else {
            // pad to full word.
            let mut fullword = [0u8; 4];
            for (b, w) in word.iter().zip(fullword.iter_mut()) {
                *w = *b;
            }

            u32::from_ne_bytes(fullword)
        };

        unsafe {
            self.fifopush.write(|f| f.bits(word));
        }
    }

    fn pop_fifo(&self, buffer: &mut [u8]) {
        for b in buffer.chunks_mut(4) {
            trace!("pop fifo: wait for word ready");
            // Wait for FIFO to fill up and commands to complete.
            while self.fifoptr.read().fifo1siz().bits() < 4 {
                if self.intstat.read().cmdcmp().bit() {
                    break;
                }

                FlashDelay::delay_us(1);
            }

            // Read a word
            trace!("pop fifo: read word.");
            let word = self.fifopop.read().bits();
            for (w, b) in word.to_ne_bytes().iter().zip(b.iter_mut()) {
                *b = *w;
            }
        }
    }
}
