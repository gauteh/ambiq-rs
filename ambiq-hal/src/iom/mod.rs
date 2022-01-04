use crate::pac;

pub mod i2c;
pub mod spi;

/// The IOM controllers support these clock speeds. See p. 269.
pub enum Freq {
    /// Standard mode
    F100kHz,

    /// Fast mode
    F400kHz,

    /// Fast mode+
    F1mHz,
}

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

/// Common functionality between the I2C and SPI modules.
pub trait Iom {
    fn wait_transfer(&mut self) -> Result<(), IomError>;

    /// Resets the I2C module and clears FIFOs.
    fn reset(&mut self);

    fn clear_interrupts(&mut self);
    fn disable_interrupts(&mut self) -> u32;
    fn enable_interrupts(&mut self, inten: u32);

    fn push_fifo(&mut self, word: &[u8]);
}

impl Iom for pac::iom0::RegisterBlock {
    fn wait_transfer(&mut self) -> Result<(), IomError> {
        defmt::trace!("wait transfer..");
        // wait for previous transfer, this check is only necessary if
        // the previous write was aborted due to e.g. a timeout.
        for _ in 0..10_000_000 {
            // loop {
            let status = self.status.read();

            if status.idlest().is_idle() && !status.cmdact().is_active() {
                defmt::trace!("wait transfer: transfer done.");
                return Ok(());
            }

            cortex_m::asm::nop();
        }

        defmt::warn!("wait transfer: timed out!");
        Err(IomError::Timeout)
    }

    fn reset(&mut self) {
        defmt::trace!("i2c: reset");

        let inten = self.disable_interrupts();

        self.wait_transfer().ok(); // XXX: Ignoring this, maybe better to reset the module if it
                                       // doesn't work?.

        // disable the submodule
        self.submodctrl.modify(|_r, w| w.smod1en().clear_bit());

        // reset FIFO
        self.fifoctrl.modify(|_r, w| w.fiforstn().clear_bit());

        defmt::trace!("i2c: reset: waiting for submodule");
        // delay for "> 6 clocks"?
        for _ in 0..10_000_000 {
            cortex_m::asm::nop();
        }
        defmt::trace!("i2c: reset: waiting for submodule: done");

        self.fifoctrl.modify(|_r, w| w.fiforstn().set_bit());
        self.submodctrl.modify(|_r, w| w.smod1en().set_bit());

        self.clear_interrupts();
        self.enable_interrupts(inten);
    }

    fn clear_interrupts(&mut self) {
        unsafe {
            self.intclr.write(|i| i.bits(0xFFFF_FFFF));
        }
    }

    fn disable_interrupts(&mut self) -> u32 {
        let inten = self.inten.read().bits();

        unsafe {
            // Disable IOM interrupts
            self.inten.write(|i| i.bits(0));

            // Disable DMA. We are doing direct writes / reads with busy polling.
            self.dmacfg.modify(|_, dw| dw.dmaen().dis());
        }

        inten
    }

    fn enable_interrupts(&mut self, inten: u32) {
        unsafe {
            self.inten.write(|i| i.bits(inten));
        }
    }

    fn push_fifo(&mut self, word: &[u8]) {
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
}
