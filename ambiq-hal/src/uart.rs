//! An UART implementation through the C-HAL.

use crate::pac;
use crate::{gpio, gpio::pin::Mode};
use crate::{halc, halc::c_types::*};
use core::ptr;
use embedded_hal as hal;
use hal::serial::{Read, Write};

pub struct Uart0 {
    /// Pointer to UART handle for Ambiq SDK.
    ph_uart: *mut c_void,

    uart: pac::UART0,

    /// Uart poins
    tx: gpio::pin::P48<{ Mode::Floating }>,
    rx: gpio::pin::P49<{ Mode::Floating }>,
}

impl Uart0 {
    pub fn new(
        uart: pac::UART0,
        tx: gpio::pin::P48<{ Mode::Floating }>,
        rx: gpio::pin::P49<{ Mode::Floating }>,
    ) -> Uart0 {
        let mut ph_uart = ptr::null_mut();

        let uart_config = halc::am_hal_uart_config_t {
            ui32BaudRate: 115200,
            ui32DataBits: halc::cAM_HAL_UART_DATA_BITS_8,
            ui32Parity: halc::cAM_HAL_UART_PARITY_NONE,
            ui32StopBits: halc::cAM_HAL_UART_ONE_STOP_BIT,
            ui32FlowControl: halc::cAM_HAL_UART_FLOW_CTRL_NONE,

            //
            // Set TX and RX FIFOs to interrupt at half-full.
            //
            ui32FifoLevels: (halc::cAM_HAL_UART_TX_FIFO_1_2 | halc::cAM_HAL_UART_RX_FIFO_1_2),

            //
            // Buffers
            //
            // Do we need 'em? What are they good for!
            pui8TxBuffer: ptr::null_mut(),
            ui32TxBufferSize: 0,
            pui8RxBuffer: ptr::null_mut(),
            ui32RxBufferSize: 0,
        };

        unsafe {
            halc::am_hal_uart_initialize(0, &mut ph_uart);
            halc::am_hal_uart_power_control(ph_uart, 0, false); // 0 = SYSTRL_WAKE
            halc::am_hal_uart_configure(ph_uart, &uart_config);

            halc::am_hal_gpio_pinconfig(tx.pin_num() as u32, halc::g_AM_BSP_GPIO_COM_UART_TX);
            halc::am_hal_gpio_pinconfig(rx.pin_num() as u32, halc::g_AM_BSP_GPIO_COM_UART_RX);
        }

        Uart0 {
            ph_uart,
            uart,
            tx,
            rx,
        }
    }
}

impl Drop for Uart0 {
    fn drop(&mut self) {
        unsafe {
            // Power down the UART, and surrender the handle.
            halc::am_hal_uart_power_control(self.ph_uart, 2, false);
            halc::am_hal_uart_deinitialize(self.ph_uart);

            // Disable the UART pins.
            halc::am_hal_gpio_pinconfig(self.tx.pin_num() as u32, halc::g_AM_HAL_GPIO_DISABLE);
            halc::am_hal_gpio_pinconfig(self.rx.pin_num() as u32, halc::g_AM_HAL_GPIO_DISABLE);

            self.ph_uart = ptr::null_mut();

            // TODO: set up a free() func to retrieve the rx,tx, etc pins back.
        }
    }
}

impl Read<u8> for Uart0 {
    type Error = ();

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        if self.uart.fr.read().rxfe().bit_is_set() {
            Err(nb::Error::WouldBlock)
        } else {
            let dr = self.uart.dr.read();
            if dr.oedata().is_err()
                || dr.bedata().is_err()
                || dr.pedata().is_err()
                || dr.fedata().is_err()
            {
                Err(nb::Error::Other(()))
            } else {
                Ok(dr.data().bits())
            }
        }
    }
}

impl Write<u8> for Uart0 {
    type Error = !;

    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        if self.uart.fr.read().txff().bit_is_set() {
            Err(nb::Error::WouldBlock)
        } else {
            unsafe {
                self.uart.dr.write(|dr| dr.data().bits(byte));
            }
            Ok(())
        }
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        // XXX: If there is a queue, also check it.

        if self.uart.fr.read().txbusy().bit_is_set() {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }
}

impl ufmt::uWrite for Uart0 {
    type Error = !;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        for b in s.as_bytes().iter() {
            nb::block!(self.write(*b))?;
        }

        Ok(())
    }
}
