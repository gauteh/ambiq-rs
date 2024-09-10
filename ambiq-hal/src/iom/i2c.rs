//! I2C IO master through C-HAL
//!
//! TODO:
//!
//!     * Disable interrupts on start, not all the time.
//!     * More sensible wait_transfer timeout or wait after reset.
//!     * DMA transfer.
//!     * Get rid of halc calls.

use super::Iom;
use core::ops::Deref;
use core::ptr;
#[allow(unused_imports)]
use defmt::{debug, error, info, trace, warn};
use embedded_hal::i2c::{self, SevenBitAddress, Operation};

use crate::gpio::{self, Mode};
use crate::delay::FlashDelay;
use crate::pac;
use crate::{halc, halc::c_types::*};

use super::Direction as I2cDirection;
pub use super::IomError as I2cError;

/// The I2C controllers support these clock speeds. See p. 269.
#[repr(u32)]
pub enum Freq {
    /// Standard mode
    F100kHz = 100_000,

    /// Fast mode
    F400kHz = 400_000,

    /// Fast mode+
    F1mHz = 1_000_000,
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

impl SdaPin<pac::IOM3> for Sda<43> {}
impl SclPin<pac::IOM3> for Scl<42> {}

impl SdaPin<pac::IOM4> for Sda<40> {}
impl SclPin<pac::IOM4> for Scl<39> {}

mod private {
    use super::{Scl, Sda};

    pub trait Sealed {}

    impl Sealed for Sda<25> {}
    impl Sealed for Scl<27> {}

    impl Sealed for Sda<43> {}
    impl Sealed for Scl<42> {}

    impl Sealed for Sda<40> {}
    impl Sealed for Scl<39> {}
}

/// QWIIC I2C controller on Redboard Nano.
pub type Iom2 = I2c<pac::IOM2, 25, 27>;

pub type Iom3 = I2c<pac::IOM3, 43, 42>;

/// QWIIC I2C controller on Redboard.
pub type Iom4 = I2c<pac::IOM4, 40, 39>;

pub struct I2c<IOM, const SDA: usize, const SCL: usize>
where
    IOM: Deref<Target = pac::iom0::RegisterBlock>,
    Sda<SDA>: SdaPin<IOM>,
    Scl<SCL>: SclPin<IOM>,
{
    phiom: *mut c_void,

    iom: IOM,

    #[allow(unused)]
    sda: gpio::pin::Pin<SDA, { Mode::Floating }>,
    #[allow(unused)]
    scl: gpio::pin::Pin<SCL, { Mode::Floating }>,
}

impl<IOM, const SDA: usize, const SCL: usize> I2c<IOM, SDA, SCL>
where
    IOM: Deref<Target = pac::iom0::RegisterBlock>,
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
                42 => 3,
                39 => 4,
                _ => unimplemented!(),
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
                defmt::debug!(
                    "Setting up pins for IOM2, SCL: {}, SDA: {}",
                    scl.pin_num(),
                    sda.pin_num()
                );
                halc::am_hal_gpio_pinconfig(scl.pin_num() as u32, halc::g_AM_BSP_GPIO_IOM2_SCL);
                halc::am_hal_gpio_pinconfig(sda.pin_num() as u32, halc::g_AM_BSP_GPIO_IOM2_SDA);
            } else if iomi == 3 {
                // IOM3
                defmt::debug!(
                    "Setting up pins for IOM3, SCL: {}, SDA: {}",
                    scl.pin_num(),
                    sda.pin_num()
                );
                halc::am_hal_gpio_pinconfig(scl.pin_num() as u32, halc::g_AM_BSP_GPIO_IOM3_SCL);
                halc::am_hal_gpio_pinconfig(sda.pin_num() as u32, halc::g_AM_BSP_GPIO_IOM3_SDA);
            } else if iomi == 4 {
                // IOM4
                defmt::debug!(
                    "Setting up pins for IOM4, SCL: {}, SDA: {}",
                    scl.pin_num(),
                    sda.pin_num()
                );
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

        self.iom.wait_transfer().ok();

        let inten = self.iom.disable_interrupts();
        self.iom.clear_interrupts();

        self.set_addr(addr.into());

        let mut words = output.chunks(4);

        // Fill up FIFO before sending command.
        while let Some(word) = words.next() {
            if self.iom.fifoptr.read().fifo0rem().bits() < 4 {
                break;
            }

            self.iom.push_fifo(word);
        }

        // Send command to start transmitting.
        self.start_tx(output.len() as u16, I2cDirection::Write, cont);

        // Push rest of bytes through FIFO
        'outer: for word in words {
            // Wait for FIFO to clear.
            while self.iom.fifoptr.read().fifo0rem().bits() < 4 {
                if self.iom.intstat.read().cmdcmp().bit() {
                    // Command completed without emptying FIFO, not good.
                    break 'outer;
                }

                FlashDelay::delay_us(1);
            }

            // Fill FIFO while there is space
            self.iom.push_fifo(word);
        }

        self.iom.wait_transfer().ok();

        // Check for errors
        let r = self.iom.check_error();

        if r.is_err() {
            error!("i2c: write: error: {:?}", r);
            self.iom.reset();
        }

        self.iom.clear_interrupts();
        self.iom.enable_interrupts(inten);

        r
    }



    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), I2cError> {
        trace!(
            "i2c: reading: addr = 0x{:02x}, len = {}",
            address,
            buffer.len()
        );

        if buffer.len() == 0 {
            return Err(I2cError::ReadTooFew);
        }

        self.iom.wait_transfer().ok();

        let inten = self.iom.disable_interrupts();
        self.iom.clear_interrupts();

        self.set_addr(address.into());

        self.start_tx(buffer.len() as u16, I2cDirection::Read, false);

        self.iom.pop_fifo(buffer)?;

        self.iom.wait_transfer().ok();

        // Check for errors
        let r = self.iom.check_error();

        if r.is_err() {
            self.iom.reset();
        }

        self.iom.clear_interrupts();
        self.iom.enable_interrupts(inten);

        r
    }
}

impl<IOM, const SDA: usize, const SCL: usize> Drop for I2c<IOM, SDA, SCL>
where
    IOM: Deref<Target = pac::iom0::RegisterBlock>,
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

impl<IOM, const SDA: usize, const SCL: usize> i2c::I2c<SevenBitAddress> for I2c<IOM, SDA, SCL>
where
    IOM: Deref<Target = pac::iom0::RegisterBlock>,
    Sda<SDA>: SdaPin<IOM>,
    Scl<SCL>: SclPin<IOM>,
    gpio::pin::Pin<SCL, { Mode::Floating }>: gpio::pin::PinCfg,
    gpio::pin::Pin<SDA, { Mode::Floating }>: gpio::pin::PinCfg,
{
    fn transaction(
        &mut self,
        address: SevenBitAddress,
        operations: &mut [Operation<'_>]
    ) -> Result<(), Self::Error> {
        for op in operations {
            match op {
                Operation::Read(buf) => self.read(address, buf)?,
                Operation::Write(buf) => self.write(address, buf, false)?,
            }
        }
    }

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
