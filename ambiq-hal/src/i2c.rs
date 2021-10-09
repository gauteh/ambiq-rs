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
    sda: gpio::pin::P40<{ Mode::Floating }>,
    #[allow(dead_code)]
    scl: gpio::pin::P39<{ Mode::Floating }>,
}

impl I2c {
    pub fn new(
        iom: pac::IOM4,
        scl: gpio::pin::P39<{ Mode::Floating }>,
        sda: gpio::pin::P40<{ Mode::Floating }>,
    ) -> I2c {
        let mut phiom = ptr::null_mut();

        let mut iomcfg = halc::am_hal_iom_config_t {
            eInterfaceMode: halc::cAM_HAL_IOM_I2CMODE,
            eSpiMode: 0,
            pNBTxnBuf: ptr::null_mut(),
            ui32NBTxnBufLength: 0,
            ui32ClockFreq: halc::AM_HAL_IOM_100KHZ,
        };

        unsafe {
            halc::am_hal_iom_initialize(4, &mut phiom); // only necessary if phiom is going to be used.
            halc::am_hal_iom_power_ctrl(phiom, 0, false); // SYSCTRL_WAKE = 0

            halc::am_hal_iom_configure(phiom, &mut iomcfg);
            halc::am_hal_iom_enable(phiom);

            // IOM ENABLE
            // iom.submodctrl.write(|w| {
            //     w.smod1type().i2c_master().smod1en().set_bit()
            // });

            halc::am_hal_gpio_pinconfig(scl.pin_num() as u32, halc::g_AM_BSP_GPIO_IOM4_SCL);
            halc::am_hal_gpio_pinconfig(sda.pin_num() as u32, halc::g_AM_BSP_GPIO_IOM4_SDA);
        }

        I2c {
            phiom,
            iom,
            scl,
            sda,
        }
    }

    fn wait_transfer(&mut self) {
        // wait for previous transfer, this check is only necessary if
        // the previous write was aborted due to e.g. a timeout.
        loop {
            let status = self.iom.status.read();

            if status.idlest().is_idle() && !status.cmdact().is_active() {
                break;
            }

            for _ in 0..1000 {} // TODO: delay 1 us
        }
    }

    fn clear_interrupts(&mut self) {
        unsafe {
            self.iom.intclr.write(|i| i.bits(0xffffffff));
        }
    }

    fn disable_interrupts(&mut self) -> u32 {
        let inten = self.iom.inten.read().bits();

        unsafe {
            // Disable IOM interrupts
            self.iom.inten.write(|i| i.bits(0));

            // Disable DMA.. ?
            self.iom.dmacfg.modify(|_, dw| dw.dmaen().dis());
        }

        inten
    }

    fn enable_interrupts(&mut self, inten: u32) {
        unsafe {
            self.iom.inten.write(|i| i.bits(inten));
        }
    }

    fn set_addr(&mut self, addr: u16, dir: I2cDirection) {
        // p. 310
        let addr = match dir {
            I2cDirection::Write => addr & 0xfe,
            I2cDirection::Read => addr | 0x01,
        };

        unsafe {
            self.iom.devcfg.write(|d| d.devaddr().bits(addr));
            self.iom.dcx.write(|d| d.bits(0));
        }
    }

    fn start_tx(&mut self, len: u16, dir: I2cDirection) {
        use ambiq_apollo3_pac::iom0::cmd::CMD_A;

        unsafe {
            // Build CMD
            self.iom.cmd.write(|cmd| {
                cmd.cmdsel().bits(0) // only for SPI CS
                    .tsize().bits(len)
                    .cmd().variant(match dir {
                        I2cDirection::Write => CMD_A::WRITE,
                        I2cDirection::Read => CMD_A::READ
                    })
                    .cont().clear_bit()
                    .offsetlo().bits(0)
                    .offsetcnt().bits(0)
            });
        }
    }

    fn reset_on_error(&mut self, e: &I2cError) {
        use I2cError::*;

        let inten = self.disable_interrupts();

        match e {
            NAK => {
                self.wait_transfer();

                // disable the submodule
                self.iom.submodctrl.modify(|_r, w| w.smod1en().clear_bit());

                // reset FIFO
                self.iom.fifoctrl.modify(|_r, w| w.fiforstn().clear_bit());

                // delay for "> 6 clocks"?
                for _ in 0..1000000 {}

                self.iom.fifoctrl.modify(|_r, w| w.fiforstn().set_bit());
                self.iom.submodctrl.modify(|_r, w| w.smod1en().set_bit());

            },
            _ => ()
        }

        self.clear_interrupts();
        self.enable_interrupts(inten);

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

enum I2cDirection {
    Write,
    Read = 1,
}

#[derive(ufmt::derive::uDebug, Copy, Clone)]
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

    Timeout,
}

impl Write<SevenBitAddress> for I2c {
    type Error = I2cError;

    fn write(&mut self, addr: u8, output: &[u8]) -> Result<(), Self::Error> {
        // TODO: XXX: Do not attempt to write more bytes than can be held in cmd.tsize() (u16)

        self.wait_transfer();
        let inten = self.disable_interrupts();
        self.clear_interrupts();

        self.set_addr(addr.into(), I2cDirection::Write);

        let mut tx_started = false;

        // Push bytes through FIFO
        for word in output.chunks(4) {
            let word = if word.len() == 4 {
                u32::from_ne_bytes(word.try_into().unwrap())
            } else {
                // pad to full word. hopefully the IOM doesn't transfer any more bytes
                // than those specifed in the CMD anyway.
                let mut fullword = [0u8; 4];
                for (b, w) in word.iter().zip(fullword.iter_mut()) {
                    *w = *b;
                }

                u32::from_ne_bytes(fullword)
            };

            while tx_started && !self.iom.intstat.read().cmdcmp().bit()
                && self.iom.fifoptr.read().fifo0rem().bits() < 4u8
            {
                for _ in 0..1000 {} // TODO: delay 1 us
            }

            unsafe {
                self.iom.fifopush.write(|f| f.bits(word));
            }

            if !tx_started {
                self.start_tx(output.len() as u16, I2cDirection::Write);
                tx_started = true;
            }
        }

        // zero-length command
        if !tx_started {
            self.start_tx(output.len() as u16, I2cDirection::Write);
        }

        self.wait_transfer();

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

        if let Err(e) = r {
            self.reset_on_error(&e);
        }

        self.clear_interrupts();
        self.enable_interrupts(inten);

        r
    }
}

impl Read<SevenBitAddress> for I2c {
    type Error = !;

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.wait_transfer();
        let inten = self.disable_interrupts();
        self.clear_interrupts();

        self.set_addr(address.into(), I2cDirection::Read);
        self.start_tx(buffer.len() as u16, I2cDirection::Read);

        for b in buffer.chunks_mut(4) {
            // Wait for FIFO to fill up and commands to complete.
            while !self.iom.intstat.read().cmdcmp().bit()
                && self.iom.fifoptr.read().fifo1siz().bits() < 4u8
            {
                for _ in 0..1000 {}
            }

            // Read a word
            let word = self.iom.fifopop.read().bits();
            for (w, b) in word.to_ne_bytes().iter().zip(b.iter_mut()) {
                *b = *w;
            }
        }

        self.wait_transfer();

        self.clear_interrupts();
        self.enable_interrupts(inten);

        Ok(())
    }
}
