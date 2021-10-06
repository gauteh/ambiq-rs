//! An UART implementation through the C-HAL.

use crate::{gpio, gpio::pin::Mode};
use crate::{halc, halc::c_types::*};
use crate::pac::interrupt;
use core::ptr;
use embedded_hal as hal;

pub struct Uart0 {
    phUART: *mut c_void,
}

fn val2fld(value: u32, field: u32, mask: u32) -> u32 {
    value << field & mask
}

impl Uart0 {
    pub fn new(
        tx: gpio::pin::P48<{ Mode::Floating }>,
        rx: gpio::pin::P49<{ Mode::Floating }>,
    ) -> Uart0 {
        let mut phUART = ptr::null_mut();

        let uart_config = halc::am_hal_uart_config_t {
            ui32BaudRate: 115200,
            ui32DataBits: val2fld(3, 5, 0x60),
            ui32Parity: 0,
            ui32StopBits: val2fld(0, 3, 0x8),
            ui32FlowControl: 0,

            //
            // Set TX and RX FIFOs to interrupt at half-full.
            //
            // ui32FifoLevels: (halc::AM_HAL_UART_TX_FIFO_1_2 | halc::AM_HAL_UART_RX_FIFO_1_2),

            ui32FifoLevels: 0,
            //
            // Buffers
            //
            pui8TxBuffer: ptr::null_mut(),
            ui32TxBufferSize: 0,
            pui8RxBuffer: ptr::null_mut(),
            ui32RxBufferSize: 0,
        };

        unsafe {
            halc::am_hal_uart_initialize(0, &mut phUART);
            halc::am_hal_uart_power_control(phUART, 0, false); // 0 = SYSTRL_WAKE
            halc::am_hal_uart_configure(phUART, &uart_config);
        }

        Uart0 { phUART }
    }
}

#[interrupt]
fn UART0() {
}
