//! I2C IO master through C-HAL
//!
//! TODO:
//!
//!     * Disable interrupts on start, not all the time.
//!     * More sensible wait_transfer timeout or wait after reset.
//!     * DMA transfer.
//!     * Get rid of halc calls.

use crate::gpio::{self, Mode};
use crate::pac;
use crate::{halc, halc::c_types::*};
use core::convert::TryInto;
use core::ops::Deref;
use core::ptr;
#[allow(unused_imports)]
use defmt::{debug, error, info, trace, warn};
use embedded_hal::blocking::i2c::*;

/// The I2C controllers support these clock speeds. See p. 269.
pub enum Freq {
    /// Standard mode
    F100kHz,

    /// Fast mode
    F400kHz,

    /// Fast mode+
    F1mHz,
}

#[derive(defmt::Format)]
enum I2cDirection {
    Write = 0x00,
    Read = 0x01,
}

#[derive(ufmt::derive::uDebug, Debug, Copy, Clone, defmt::Format)]
pub enum I2cError {
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

// This is an attempt at preventing users from instantiating an IOM with different pins than those
// that can be used.
#[doc(hidden)]
pub struct Sda<const P: usize>;
#[doc(hidden)]
pub struct Scl<const P: usize>;

#[doc(hidden)]
pub trait SdaPin<T>: private::Sealed {}
#[doc(hidden)]
pub trait SclPin<T>: private::Sealed {}

impl SdaPin<pac::IOM2> for Sda<25> {}
impl SclPin<pac::IOM2> for Scl<27> {}
impl SdaPin<pac::IOM4> for Sda<40> {}
impl SclPin<pac::IOM4> for Scl<39> {}

mod private {
    use super::{Sda, Scl};

    pub trait Sealed {}

    impl Sealed for Sda<25> {}
    impl Sealed for Scl<27> {}
    impl Sealed for Sda<40> {}
    impl Sealed for Scl<39> {}
}


/// QWIIC I2C controller on Redboard Nano.
pub type Iom2 = I2c<pac::IOM2, 25, 27>;

/// QWIIC I2C controller on Redboard.
pub type Iom4 = I2c<pac::IOM4, 40, 39>;

pub struct I2c<IOM: Deref<Target = pac::iom0::RegisterBlock>, const SDA: usize, const SCL: usize>
where
    Sda<SDA>: SdaPin<IOM>,
    Scl<SCL>: SclPin<IOM>
{
    phiom: *mut c_void,

    iom: IOM,

    #[allow(unused)]
    sda: gpio::pin::Pin<SDA, { Mode::Floating }>,
    #[allow(unused)]
    scl: gpio::pin::Pin<SCL, { Mode::Floating }>,
}

impl<IOM: Deref<Target = pac::iom0::RegisterBlock>, const SDA: usize, const SCL: usize>
    I2c<IOM, SDA, SCL>
where
    gpio::pin::Pin<SCL, { Mode::Floating }>: gpio::pin::PinCfg,
    gpio::pin::Pin<SDA, { Mode::Floating }>: gpio::pin::PinCfg,
    Sda<SDA>: SdaPin<IOM>,
    Scl<SCL>: SclPin<IOM>,
{
    pub fn new(
        iom: IOM,
        sda: gpio::pin::Pin<SDA, { Mode::Floating }>,
        scl: gpio::pin::Pin<SCL, { Mode::Floating }>,
        freq: Freq,
    ) -> I2c<IOM, SDA, SCL> {
        let mut phiom = ptr::null_mut();

        let freq = match freq {
            Freq::F100kHz => halc::AM_HAL_IOM_100KHZ,
            Freq::F400kHz => halc::AM_HAL_IOM_400KHZ,
            Freq::F1mHz => halc::AM_HAL_IOM_1MHZ,
        };

        let mut iomcfg = halc::am_hal_iom_config_t {
            eInterfaceMode: halc::cAM_HAL_IOM_I2CMODE,
            eSpiMode: 0,
            pNBTxnBuf: ptr::null_mut(),
            ui32NBTxnBufLength: 0,
            ui32ClockFreq: freq,
        };

        unsafe {
            let iomi = match scl.pin_num() {
                27 => 2,
                39 => 4,
                _ => unimplemented!()
            };

            halc::am_hal_iom_initialize(iomi, &mut phiom); // only necessary if phiom is going to be used.
            halc::am_hal_iom_power_ctrl(phiom, 0, false); // SYSCTRL_WAKE = 0

            halc::am_hal_iom_configure(phiom, &mut iomcfg);
            halc::am_hal_iom_enable(phiom);

            // IOM ENABLE
            // iom.submodctrl.write(|w| {
            //     w.smod1type().i2c_master().smod1en().set_bit()
            // });

            // Let's get rid of this stuff asap
            if iomi == 2 {
                // IOM2
                defmt::debug!("Setting up pins for IOM2, SCL: {}, SDA: {}", scl.pin_num(), sda.pin_num());
                halc::am_hal_gpio_pinconfig(scl.pin_num() as u32, halc::g_AM_BSP_GPIO_IOM2_SCL);
                halc::am_hal_gpio_pinconfig(sda.pin_num() as u32, halc::g_AM_BSP_GPIO_IOM2_SDA);
            } else if iomi == 4 {
                // IOM4
                defmt::debug!("Setting up pins for IOM4, SCL: {}, SDA: {}", scl.pin_num(), sda.pin_num());
                halc::am_hal_gpio_pinconfig(scl.pin_num() as u32, halc::g_AM_BSP_GPIO_IOM4_SCL);
                halc::am_hal_gpio_pinconfig(sda.pin_num() as u32, halc::g_AM_BSP_GPIO_IOM4_SDA);
            } else {
                unimplemented!()
            }
        }

        I2c {
            phiom,
            iom,
            scl,
            sda,
        }
    }

    fn wait_transfer(&mut self) -> Result<(), I2cError> {
        defmt::trace!("wait transfer..");
        // wait for previous transfer, this check is only necessary if
        // the previous write was aborted due to e.g. a timeout.
        for _ in 0..10_000_000 {
            // loop {
            let status = self.iom.status.read();

            if status.idlest().is_idle() && !status.cmdact().is_active() {
                defmt::trace!("wait transfer: transfer done.");
                return Ok(());
            }

            cortex_m::asm::nop();
        }

        defmt::warn!("wait transfer: timed out!");
        Err(I2cError::Timeout)
    }

    fn clear_interrupts(&mut self) {
        unsafe {
            self.iom.intclr.write(|i| i.bits(0xFFFF_FFFF));
        }
    }

    fn disable_interrupts(&mut self) -> u32 {
        let inten = self.iom.inten.read().bits();

        unsafe {
            // Disable IOM interrupts
            self.iom.inten.write(|i| i.bits(0));

            // Disable DMA. We are doing direct writes / reads with busy polling.
            self.iom.dmacfg.modify(|_, dw| dw.dmaen().dis());
        }

        inten
    }

    fn enable_interrupts(&mut self, inten: u32) {
        unsafe {
            self.iom.inten.write(|i| i.bits(inten));
        }
    }

    fn set_addr(&mut self, addr: u16) {
        // p. 310

        unsafe {
            self.iom.devcfg.write(|d| d.devaddr().bits(addr));
            self.iom.dcx.write(|d| d.bits(0));
        }
    }

    fn start_tx(&mut self, len: u16, dir: I2cDirection, cont: bool) {
        use pac::iom0::cmd::CMD_A;

        trace!("start tx (len={}) (dir={})", &len, dir);

        unsafe {
            // Build CMD
            self.iom.cmd.write(|cmd| {
                cmd.cmdsel()
                    .bits(0) // only for SPI CS
                    .tsize()
                    .bits(len)
                    .cmd()
                    .variant(match dir {
                        I2cDirection::Write => CMD_A::WRITE,
                        I2cDirection::Read => CMD_A::READ,
                    })
                    .cont()
                    .bit(cont)
                    .offsetlo()
                    .bits(0)
                    .offsetcnt()
                    .bits(0)
            });
        }
    }

    /// Resets the I2C module and clears FIFOs.
    pub fn reset(&mut self) {
        defmt::trace!("i2c: reset");

        let inten = self.disable_interrupts();

        self.wait_transfer().ok(); // XXX: Ignoring this, maybe better to reset the module if it
                                   // doesn't work?.

        // disable the submodule
        self.iom.submodctrl.modify(|_r, w| w.smod1en().clear_bit());

        // reset FIFO
        self.iom.fifoctrl.modify(|_r, w| w.fiforstn().clear_bit());

        defmt::trace!("i2c: reset: waiting for submodule");
        // delay for "> 6 clocks"?
        for _ in 0..10_000_000 {
            cortex_m::asm::nop();
        }
        defmt::trace!("i2c: reset: waiting for submodule: done");

        self.iom.fifoctrl.modify(|_r, w| w.fiforstn().set_bit());
        self.iom.submodctrl.modify(|_r, w| w.smod1en().set_bit());

        self.clear_interrupts();
        self.enable_interrupts(inten);
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
            self.iom.fifopush.write(|f| f.bits(word));
        }
    }

    /// Pings the address by performing a zero-byte write.
    pub fn ping(&mut self, addr: u8) -> bool {
        self.write(addr, &[], false).is_ok()
    }

    fn write(&mut self, addr: u8, output: &[u8], cont: bool) -> Result<(), I2cError> {
        // TODO: XXX: Do not attempt to write more bytes than can be held in cmd.tsize() (u16), or 255 bytes?
        trace!(
            "i2c: writing: addr = 0x{:02x}, len = {}",
            addr,
            output.len()
        );

        self.wait_transfer().ok();

        let inten = self.disable_interrupts();
        self.clear_interrupts();

        self.set_addr(addr.into());

        let mut words = output.chunks(4);

        // Fill up FIFO before sending command.
        while let Some(word) = words.next() {
            if self.iom.fifoptr.read().fifo0rem().bits() < 4 {
                break;
            }

            self.push_fifo(word);
        }

        // Send command to start transmitting.
        self.start_tx(output.len() as u16, I2cDirection::Write, cont);

        // Push rest of bytes through FIFO
        'outer: for word in words {
            // Wait for FIFO to clear.
            while self.iom.fifoptr.read().fifo0rem().bits() < 4 {
                cortex_m::asm::nop();

                if self.iom.intstat.read().cmdcmp().bit() {
                    // Command completed without emptying FIFO, not good.
                    break 'outer;
                }
            }

            // Fill FIFO while there is space
            self.push_fifo(word);
        }

        self.wait_transfer().ok();

        // Check for errors
        let r = match self.iom.intstat.read() {
            i if i.icmd().bit() || i.fovfl().bit() || i.fundfl().bit() || i.iacc().bit() => {
                Err(I2cError::SwError)
            }
            i if i.arb().bit() || i.start().bit() || i.stop().bit() => Err(I2cError::ARB),
            i if i.nak().bit() => Err(I2cError::NAK),
            i if i.cqerr().bit() || i.derr().bit() => Err(I2cError::Other),
            _ => Ok(()),
        };

        if r.is_err() {
            error!("i2c: write: error: {:?}", r);
            self.reset();
        }

        self.clear_interrupts();
        self.enable_interrupts(inten);

        r
    }
}

impl<IOM: Deref<Target = pac::iom0::RegisterBlock>, const SDA: usize, const SCL: usize> Drop
    for I2c<IOM, SDA, SCL>
where
    Sda<SDA>: SdaPin<IOM>,
    Scl<SCL>: SclPin<IOM>,
{
    fn drop(&mut self) {
        unsafe {
            halc::am_hal_iom_uninitialize(self.phiom);
            self.phiom = ptr::null_mut();
        }
    }
}

impl<IOM: Deref<Target = pac::iom0::RegisterBlock>, const SDA: usize, const SCL: usize>
    Write<SevenBitAddress> for I2c<IOM, SDA, SCL>
where
    Sda<SDA>: SdaPin<IOM>,
    Scl<SCL>: SclPin<IOM>,
    gpio::pin::Pin<SCL, { Mode::Floating }>: gpio::pin::PinCfg,
    gpio::pin::Pin<SDA, { Mode::Floating }>: gpio::pin::PinCfg,
{
    type Error = I2cError;

    fn write(&mut self, addr: u8, output: &[u8]) -> Result<(), Self::Error> {
        self.write(addr, output, false)
    }
}

impl<IOM: Deref<Target = pac::iom0::RegisterBlock>, const SDA: usize, const SCL: usize>
    Read<SevenBitAddress> for I2c<IOM, SDA, SCL>
where
    Sda<SDA>: SdaPin<IOM>,
    Scl<SCL>: SclPin<IOM>,
    gpio::pin::Pin<SCL, { Mode::Floating }>: gpio::pin::PinCfg,
    gpio::pin::Pin<SDA, { Mode::Floating }>: gpio::pin::PinCfg,
{
    type Error = I2cError;

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        trace!(
            "i2c: reading: addr = 0x{:02x}, len = {}",
            address,
            buffer.len()
        );

        if buffer.len() == 0 {
            return Err(I2cError::ReadTooFew);
        }

        self.wait_transfer().ok();

        let inten = self.disable_interrupts();
        self.clear_interrupts();

        self.set_addr(address.into());

        self.start_tx(buffer.len() as u16, I2cDirection::Read, false);

        for b in buffer.chunks_mut(4) {
            // Wait for FIFO to fill up and commands to complete.
            while self.iom.fifoptr.read().fifo1siz().bits() < 4 {
                cortex_m::asm::nop();

                if self.iom.intstat.read().cmdcmp().bit() {
                    break;
                }
            }

            // Read a word
            let word = self.iom.fifopop.read().bits();
            for (w, b) in word.to_ne_bytes().iter().zip(b.iter_mut()) {
                *b = *w;
            }
        }

        self.wait_transfer().ok();

        // Check for errors
        let r = match self.iom.intstat.read() {
            i if i.icmd().bit() || i.fovfl().bit() || i.fundfl().bit() || i.iacc().bit() => {
                Err(I2cError::SwError)
            }
            i if i.arb().bit() || i.start().bit() || i.stop().bit() => Err(I2cError::ARB),
            i if i.nak().bit() => Err(I2cError::NAK),
            i if i.cqerr().bit() || i.derr().bit() => Err(I2cError::Other),
            _ => Ok(()),
        };

        if r.is_err() {
            self.reset();
        }

        self.clear_interrupts();
        self.enable_interrupts(inten);

        r
    }
}

impl<IOM: Deref<Target = pac::iom0::RegisterBlock>, const SDA: usize, const SCL: usize>
    WriteRead<SevenBitAddress> for I2c<IOM, SDA, SCL>
where
    Sda<SDA>: SdaPin<IOM>,
    Scl<SCL>: SclPin<IOM>,
    gpio::pin::Pin<SCL, { Mode::Floating }>: gpio::pin::PinCfg,
    gpio::pin::Pin<SDA, { Mode::Floating }>: gpio::pin::PinCfg,
{
    type Error = I2cError;

    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        // Write _with_ setting CONT
        self.write(address, bytes, true)?;

        // Read _without_ setting CONT (exactly same as Read)
        self.read(address, buffer)?;

        Ok(())
    }
}
