use pac::GPIO;

pub use super::pin::*;

pub struct Pins {
    _gpio: GPIO,

    /// * GPIO d10
    /// * PWM
    pub d10: P13<{ Mode::Floating }>,

    /// * GPIO d13
    /// * Blue LED (Redboard Artemis)
    /// * SCK0 (SPI0)
    pub d13: P5<{ Mode::Floating }>,

    /// * MISO0 (SPI0)
    pub d12: P6<{ Mode::Floating }>,

    /// * MOSI0 (SPI0)
    pub d11: P7<{ Mode::Floating }>,

    /// * USART0 TX (Serial-over-USB)
    pub tx0: P48<{ Mode::Floating }>,

    /// * USART0 RX (Serial-over-USB)
    pub rx0: P49<{ Mode::Floating }>,

    /// * QWIIC SDA (Redboard Artemis)
    /// * SDA4
    /// * MISO4
    /// * RX1
    pub d14: P40<{ Mode::Floating }>,

    /// * QWIIC SCL (Redboard Artemis)
    /// * SCL4
    /// * SCK4
    /// * TX1
    pub d15: P39<{ Mode::Floating }>,

    /// * SDA2
    pub d17: P25<{ Mode::Floating }>,

    /// * SCL2
    /// * SCK2
    pub d18: P27<{ Mode::Floating }>,

    /// * SCL3
    /// * TX1
    pub d7: P42<{ Mode::Floating }>,

    /// * SDA3
    /// * RX1
    pub d6: P43<{ Mode::Floating }>,

    /// * GPIO d19
    pub d19: P19<{ Mode::Floating }>,
}

impl Pins {
    pub fn new(gpio: GPIO) -> Pins {
        Pins {
            _gpio: gpio,
            d6: Pin::new(),
            d7: Pin::new(),
            d10: Pin::new(),
            d11: Pin::new(),
            d12: Pin::new(),
            d13: Pin::new(),
            d14: Pin::new(),
            d15: Pin::new(),
            d17: Pin::new(),
            d18: Pin::new(),
            d19: Pin::new(),
            tx0: Pin::new(),
            rx0: Pin::new(),
        }
    }
}
