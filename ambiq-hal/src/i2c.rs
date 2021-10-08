//! I2C IO master through C-HAL

use crate::gpio::{self, Mode};
use crate::pac;
use crate::{halc, halc::c_types::*};
use core::convert::TryInto;
use core::ptr;
use embedded_hal::blocking::i2c::*;

/// The QWIIC I2C controller.
pub struct I2c {
    phiom: *mut c_void,

    iom: pac::IOM4,
    #[allow(dead_code)]
    scl: gpio::pin::P40<{ Mode::Floating }>,
    #[allow(dead_code)]
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
            sda,
        }
    }
}

impl Drop for I2c {
    fn drop(&mut self) {
        unsafe {
            halc::am_hal_iom_uninitialize(self.phiom);
            self.phiom = ptr::null_mut();
        }
    }
}

impl Write<SevenBitAddress> for I2c {
    type Error = !;

    fn write(&mut self, addr: u8, output: &[u8]) -> Result<(), Self::Error> {
        // wait for previous transfer, this check is only necessary if
        // the previous write was aborted due to e.g. a timeout.
        loop {
            let status = self.iom.status.read();
            if status.idlest().is_idle() && !status.cmdact().is_active() {
                break;
            }

            for _ in 0..1000 {} // TODO: delay 1 us
        }

        unsafe {
            // Disable IOM interrupts
            self.iom.inten.write(|i| i.bits(0));

            // Disable DMA.. ?
            self.iom.dmacfg.modify(|_, dw| dw.dmaen().dis());

            // Clear interrupts
            self.iom.intclr.write(|i| i.bits(0xFFFF));

            // Set address
            self.iom.devcfg.write(|d| d.devaddr().bits(addr.into()));

            self.iom.dcx.reset();

            // Build CMD
            self.iom.cmd.write(|cmd| {
                cmd.cmdsel().bits(0); // only for SPI CS
                cmd.tsize().bits(output.len() as u16);
                cmd.cmd().write();
                cmd.cont().clear_bit();

                cmd.offsetlo().bits(0);
                cmd.offsetcnt().bits(0);

                cmd
            });
        }

        // Push bytes through FIFO
        let words = output.chunks(4);
        for word in words {
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

            while self.iom.fifoptr.read().fifo0rem().bits() >= 4u8 {
                for _ in 0..1000 {} // TODO: delay 1 us
            }

            unsafe {
                self.iom.fifopush.write(|f| f.bits(word));
            }
        }

        Ok(())
    }
}
