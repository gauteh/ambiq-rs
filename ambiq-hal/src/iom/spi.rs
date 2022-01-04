use core::ops::{Deref, DerefMut};
use core::ptr;
#[allow(unused_imports)]
use defmt::{debug, error, info, trace, warn};
use embedded_hal::spi::FullDuplex;

use crate::gpio::{self, Mode};
use crate::pac;
use crate::{halc, halc::c_types::*};

use super::Direction as I2cDirection;
pub use super::Freq;
pub use super::IomError as I2cError;

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
    IOM: Deref<Target = pac::iom0::RegisterBlock> + DerefMut<Target = pac::iom0::RegisterBlock>,
    gpio::pin::Pin<MOSI, { Mode::Floating }>: gpio::pin::PinCfg,
    gpio::pin::Pin<MISO, { Mode::Floating }>: gpio::pin::PinCfg,
    gpio::pin::Pin<SCK, { Mode::Floating }>: gpio::pin::PinCfg,
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
}

impl<IOM, const MOSI: usize, const MISO: usize, const SCK: usize> Spi<IOM, MOSI, MISO, SCK>
where
    IOM: Deref<Target = pac::iom0::RegisterBlock> + DerefMut<Target = pac::iom0::RegisterBlock>,
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
    ) -> Spi<IOM, MOSI, MISO, SCK> {
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
            let iomi = match sck.pin_num() {
                5 => 0,
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
            if iomi == 0 {
                // IOM0
                defmt::debug!("Setting up pins for IOM0");
                halc::am_hal_gpio_pinconfig(sck.pin_num() as u32, halc::g_AM_BSP_GPIO_IOM0_SCK);
                halc::am_hal_gpio_pinconfig(mosi.pin_num() as u32, halc::g_AM_BSP_GPIO_IOM0_MOSI);
                halc::am_hal_gpio_pinconfig(miso.pin_num() as u32, halc::g_AM_BSP_GPIO_IOM0_MISO);
            } else {
                unimplemented!()
            }
        }

        Spi {
            phiom,
            iom,
            mosi,
            miso,
            sck,
        }
    }
}
