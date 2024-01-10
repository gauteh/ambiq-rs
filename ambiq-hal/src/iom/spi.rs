use core::ops::Deref;
use core::ptr;
#[allow(unused_imports)]
use defmt::{debug, error, info, trace, warn};
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::spi::{self, FullDuplex};

use crate::gpio::{self, Mode};
use crate::delay::FlashDelay;
use crate::pac;
use crate::{halc, halc::c_types::*};

use super::{Direction, Iom, IomError};

/// The SPI controllers support these clock speeds. See p. 269.
#[derive(defmt::Format)]
#[repr(u32)]
pub enum Freq {
    F10kHz = 10_000,
    F50kHz = 50_000,
    F100kHz = 100_000,
    F125kHz = 125_000,
    F250kHz = 250_000,
    F375kHz = 375_000,
    F400kHz = 400_000,
    F500kHz = 500_000,
    F750kHz = 750_000,
    F1mHz = 1_000_000,
    F1_5mHz = 1_500_000,
    F2mHz = 2_000_000,
    F3mHz = 3_000_000,
    F4mHz = 4_000_000,
    F6mHz = 6_000_000,
    F8mHz = 8_000_000,
    F12mHz = 12_000_000,
    F16mHz = 16_000_000,
    F24mHz = 24_000_000,
    F48mHz = 48_000_000,
}

// This prevents an IOM from being instantiated with incorrect pins.
#[doc(hidden)]
pub struct Mosi<const P: usize>;
#[doc(hidden)]
pub struct Miso<const P: usize>;
#[doc(hidden)]
pub struct Sck<const P: usize>;

#[doc(hidden)]
pub trait MosiPin<T>: private::Sealed {}
#[doc(hidden)]
pub trait MisoPin<T>: private::Sealed {}
#[doc(hidden)]
pub trait SckPin<T>: private::Sealed {}

impl MosiPin<pac::IOM0> for Mosi<7> {}
impl MisoPin<pac::IOM0> for Miso<6> {}
impl SckPin<pac::IOM0> for Sck<5> {}

mod private {
    use super::{Miso, Mosi, Sck};

    pub trait Sealed {}

    impl Sealed for Mosi<7> {}
    impl Sealed for Miso<6> {}
    impl Sealed for Sck<5> {}
}

/// SCK, MISO and MOSI on Redboard.
pub type Spi0 = Spi<pac::IOM0, 7, 6, 5>;

pub struct Spi<IOM, const MOSI: usize, const MISO: usize, const SCK: usize>
where
    IOM: Deref<Target = pac::iom0::RegisterBlock>,
    Mosi<MOSI>: MosiPin<IOM>,
    Miso<MISO>: MisoPin<IOM>,
    Sck<SCK>: SckPin<IOM>,
{
    phiom: *mut c_void,
    iom: IOM,

    #[allow(unused)]
    mosi: gpio::pin::Pin<MOSI, { Mode::Floating }>,
    #[allow(unused)]
    miso: gpio::pin::Pin<MISO, { Mode::Floating }>,
    #[allow(unused)]
    sck: gpio::pin::Pin<SCK, { Mode::Floating }>,

    mode: spi::Mode,
}

impl<IOM, const MOSI: usize, const MISO: usize, const SCK: usize> Drop for Spi<IOM, MOSI, MISO, SCK>
where
    IOM: Deref<Target = pac::iom0::RegisterBlock>,
    Mosi<MOSI>: MosiPin<IOM>,
    Miso<MISO>: MisoPin<IOM>,
    Sck<SCK>: SckPin<IOM>,
{
    fn drop(&mut self) {
        unsafe {
            halc::am_hal_iom_uninitialize(self.phiom);
            self.phiom = ptr::null_mut();
        }
    }
}

impl<IOM, const MOSI: usize, const MISO: usize, const SCK: usize> Spi<IOM, MOSI, MISO, SCK>
where
    IOM: Deref<Target = pac::iom0::RegisterBlock>,
    gpio::pin::Pin<MOSI, { Mode::Floating }>: gpio::pin::PinCfg,
    gpio::pin::Pin<MISO, { Mode::Floating }>: gpio::pin::PinCfg,
    gpio::pin::Pin<SCK, { Mode::Floating }>: gpio::pin::PinCfg,
    Mosi<MOSI>: MosiPin<IOM>,
    Miso<MISO>: MisoPin<IOM>,
    Sck<SCK>: SckPin<IOM>,
{
    pub fn new(
        iom: IOM,
        mosi: gpio::pin::Pin<MOSI, { Mode::Floating }>,
        miso: gpio::pin::Pin<MISO, { Mode::Floating }>,
        sck: gpio::pin::Pin<SCK, { Mode::Floating }>,
        freq: Freq,
        mode: spi::Mode,
    ) -> Spi<IOM, MOSI, MISO, SCK> {
        let phiom = ptr::null_mut();

        let mut spi = Spi {
            phiom,
            iom,
            mosi,
            miso,
            sck,
            mode,
        };

        spi.initialize(freq);

        spi
    }

    fn initialize(&mut self, freq: Freq) {
        // TODO: Can apparently support a much wider (and higher) range of frequencies.
        let freq = freq as u32;
        let spimode = match self.mode {
            spi::MODE_0 => 0,
            spi::MODE_1 => 1,
            spi::MODE_2 => 2,
            spi::MODE_3 => 3,
        };

        let mut iomcfg = halc::am_hal_iom_config_t {
            eInterfaceMode: halc::cAM_HAL_IOM_SPIMODE,
            eSpiMode: spimode,
            pNBTxnBuf: ptr::null_mut(),
            ui32NBTxnBufLength: 0,
            ui32ClockFreq: freq,
        };

        unsafe {
            let iomi = match self.sck.pin_num() {
                5 => 0,
                _ => unimplemented!(),
            };

            halc::am_hal_iom_initialize(iomi, &mut self.phiom); // only necessary if phiom is going to be used.
            halc::am_hal_iom_power_ctrl(self.phiom, 0, false); // SYSCTRL_WAKE = 0

            halc::am_hal_iom_configure(self.phiom, &mut iomcfg);
            halc::am_hal_iom_enable(self.phiom);

            // IOM ENABLE
            // iom.submodctrl.write(|w| {
            //     w.smod1type().i2c_master().smod1en().set_bit()
            // });

            // Let's get rid of this stuff asap
            if iomi == 0 {
                // IOM0
                defmt::debug!("Setting up pins for IOM0");
                halc::am_hal_gpio_pinconfig(
                    self.sck.pin_num() as u32,
                    halc::g_AM_BSP_GPIO_IOM0_SCK,
                );
                halc::am_hal_gpio_pinconfig(
                    self.mosi.pin_num() as u32,
                    halc::g_AM_BSP_GPIO_IOM0_MOSI,
                );
                halc::am_hal_gpio_pinconfig(
                    self.miso.pin_num() as u32,
                    halc::g_AM_BSP_GPIO_IOM0_MISO,
                );
            } else {
                unimplemented!()
            }
        }
    }

    pub fn set_freq(&mut self, freq: Freq) {
        defmt::debug!("Setting frequency to: {:?}", freq);
        unsafe {
            halc::am_hal_iom_uninitialize(self.phiom);
            self.phiom = ptr::null_mut();
        }

        self.initialize(freq);
    }

    fn start_tx(&mut self, len: u16, dir: Direction, cont: bool) {
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
                        Direction::Write => CMD_A::WRITE,
                        Direction::Read => CMD_A::READ,
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
}

// Each `read` _must_ be preceded by a `send` call. The Ambiq does full-duplex in one go, so you won't
// actually send anything before `read` is called.
//
// `send` initiates the command, and pushes a word to the FIFO. `read` completes the command by
// popping the read word from the FIFO.
impl<IOM, const MOSI: usize, const MISO: usize, const SCK: usize> FullDuplex<u8>
    for Spi<IOM, MOSI, MISO, SCK>
where
    IOM: Deref<Target = pac::iom0::RegisterBlock>,
    gpio::pin::Pin<MOSI, { Mode::Floating }>: gpio::pin::PinCfg,
    gpio::pin::Pin<MISO, { Mode::Floating }>: gpio::pin::PinCfg,
    gpio::pin::Pin<SCK, { Mode::Floating }>: gpio::pin::PinCfg,
    Mosi<MOSI>: MosiPin<IOM>,
    Miso<MISO>: MisoPin<IOM>,
    Sck<SCK>: SckPin<IOM>,
{
    type Error = IomError;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        if self.iom.is_ready() {
            trace!("spi: full-duplex: reading byte");
            let inten = self.iom.disable_interrupts();
            self.iom.clear_interrupts();

            let mut buf = [0u8];
            self.iom.pop_fifo(&mut buf)?;

            // Check for errors
            let r = self.iom.check_error();

            if r.is_err() {
                self.iom.reset();
            }

            self.iom.clear_interrupts();
            self.iom.enable_interrupts(inten);

            Ok(buf[0])
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn send(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        if self.iom.is_ready() {
            trace!("spi: full-duplex: writing byte: {:x}", word);
            let inten = self.iom.disable_interrupts();
            self.iom.clear_interrupts();

            // set Full-Duplex mode
            self.iom.mspicfg.write(|w| w.fulldup().set_bit());
            self.start_tx(1, Direction::Write, false);

            self.iom.push_fifo(&[word]);

            // Check for errors
            let r = self.iom.check_error();

            if r.is_err() {
                self.iom.reset();
            }

            self.iom.clear_interrupts();
            self.iom.enable_interrupts(inten);

            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<IOM, const MOSI: usize, const MISO: usize, const SCK: usize> hal::blocking::spi::write::Default<u8>
    for Spi<IOM, MOSI, MISO, SCK>
where
    IOM: Deref<Target = pac::iom0::RegisterBlock>,
    gpio::pin::Pin<MOSI, { Mode::Floating }>: gpio::pin::PinCfg,
    gpio::pin::Pin<MISO, { Mode::Floating }>: gpio::pin::PinCfg,
    gpio::pin::Pin<SCK, { Mode::Floating }>: gpio::pin::PinCfg,
    Mosi<MOSI>: MosiPin<IOM>,
    Miso<MISO>: MisoPin<IOM>,
    Sck<SCK>: SckPin<IOM>,
{}

impl<IOM, const MOSI: usize, const MISO: usize, const SCK: usize> Transfer<u8>
    for Spi<IOM, MOSI, MISO, SCK>
where
    IOM: Deref<Target = pac::iom0::RegisterBlock>,
    gpio::pin::Pin<MOSI, { Mode::Floating }>: gpio::pin::PinCfg,
    gpio::pin::Pin<MISO, { Mode::Floating }>: gpio::pin::PinCfg,
    gpio::pin::Pin<SCK, { Mode::Floating }>: gpio::pin::PinCfg,
    Mosi<MOSI>: MosiPin<IOM>,
    Miso<MISO>: MisoPin<IOM>,
    Sck<SCK>: SckPin<IOM>,
{
    type Error = IomError;
    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        self.iom.wait_transfer().ok();

        let inten = self.iom.disable_interrupts();
        self.iom.clear_interrupts();

        // set Full-Duplex mode
        self.iom.mspicfg.write(|w| w.fulldup().set_bit());

        // Write
        trace!("spi: full-duplex: write: {:x}", &words);
        for chunk in words.chunks_mut(4) {
            self.start_tx(chunk.len() as u16, Direction::Write, false);
            trace!("spi: full-duplex: chunk: {:x}", &chunk);
            self.iom.push_fifo(chunk);

            if self.iom.wait_transfer().is_err() {
                self.iom.reset();
            }

            // Read
            self.iom.pop_fifo(chunk)?;
            trace!("spi: full-duplex: read: {:x}", &chunk);

            // Check for errors
            let r = self.iom.check_error();

            if r.is_err() {
                self.iom.reset();
            }
        }

        self.iom.clear_interrupts();
        self.iom.enable_interrupts(inten);

        Ok(words)
    }
}
