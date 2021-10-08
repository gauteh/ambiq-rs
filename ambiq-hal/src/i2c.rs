//! I2C IO master through C-HAL

use crate::gpio::{self, Mode};
use crate::{halc, halc::c_types::*};
use crate::pac;
use core::ptr;

/// The QWIIC I2C controller.
pub struct I2c {
    phiom: * mut c_void,

    iom: pac::IOM4,
    scl: gpio::pin::P40<{ Mode::Floating }>,
    sda: gpio::pin::P39<{ Mode::Floating }>,
}

impl I2c {
    pub fn new(
        iom: pac::IOM4,
        scl: gpio::pin::P40<{ Mode::Floating }>,
        sda: gpio::pin::P39<{ Mode::Floating }>,
    ) -> I2c {
        let mut phiom = ptr::null_mut();

        let mut iomcfg = halc::am_hal_iom_config_t {
            eInterfaceMode: halc::cAM_HAL_IOM_I2CMODE,
            eSpiMode: 0,
            pNBTxnBuf: ptr::null_mut(),
            ui32NBTxnBufLength: 0,
            ui32ClockFreq: halc::cAM_HAL_IOM_400KHZ,
        };

        unsafe {
            halc::am_hal_iom_initialize(4, &mut phiom); // only necessary if phiom is going to be used.
            halc::am_hal_iom_power_ctrl(phiom, 0, false); // SYSCTRL_WAKE = 0
            halc::am_hal_iom_configure(phiom, &mut iomcfg);
            halc::am_hal_iom_enable(phiom);

            halc::am_hal_gpio_pinconfig(scl.pin_num() as u32, halc::g_AM_BSP_GPIO_IOM2_SCL);
            halc::am_hal_gpio_pinconfig(sda.pin_num() as u32, halc::g_AM_BSP_GPIO_IOM2_SDA);
        }

        I2c {
            phiom,
            iom,
            scl,
            sda }
    }
}
