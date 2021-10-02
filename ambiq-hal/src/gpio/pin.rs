use core::marker::PhantomData;
use pac::GPIO;
use paste::paste;

// The GPIO_PADKEY register must be set to 0x73 before writing to the PADREGn registers,
// and should be cleared (or set to another value) afterwards.
const PAD_KEY: u32 = 0x73;

pub(crate) fn gpio_cfg<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    cortex_m::interrupt::free(|_| {
        // page 419
        unsafe {
            (*pac::GPIO::ptr()).padkey.write(|w| w.bits(PAD_KEY));
        }

        let r = f();

        unsafe {
            (*pac::GPIO::ptr()).padkey.write(|w| w.bits(0x00));
        }

        r
    })
}

/// The drive strength is controlled by setting registers:
/// ALTPADCFGy_PADn_DS1 and PADREGy_PADnSTRNG.
///
/// Table 597, p. 420.
#[derive(PartialEq, Eq)]
pub enum DriveStrength {
    D2mA,
    D4mA,
    D8mA,
    D12mA,
}

#[derive(PartialEq, Eq)]
pub enum OutputMode {
    Disable = 0,
    PushPull,
    OpenDrain,
    TriState,
}

#[derive(PartialEq, Eq)]
pub enum InputMode {
    NoneOrAuto = 0, // These are two states, but they map to the same value.
    Enable = 1,
}

#[derive(PartialEq, Eq)]
pub enum Mode {
    Floating,
    Input,
    Output,
}

pub struct Pin<const PINNUM: usize, const MODE: Mode> {}

pub trait PinCfg {
    fn padpull(&mut self, on: bool);
}

macro_rules! pin {
    ($id:literal, $padreg:ident) => {
        paste! {
            pub type [<P $id>]<const MODE: Mode> = Pin<$id, MODE>;

            impl<const MODE: Mode>PinCfg for Pin<$id, MODE> {
                fn padpull(&mut self, on: bool) {

                    // XXX: Pad 20 differs in behavior to the rest of the pads, see p. 420.

                    gpio_cfg(|| unsafe {
                        (*pac::GPIO::ptr()).[<padreg $padreg:lower>]
                            .write(|p| p.[< pad $id pull >]().bit(on))
                    })
                }
            }
        }
    };
}


impl<const P: usize> Pin<P, { Mode::Floating }> {
    pub fn new() -> Self {
        Self {}
    }
}

impl<const P: usize, const M: Mode> Pin<P, M> {
    pub fn with_mode() -> Self {
        let mut p = Self {};
        p.mode::<M>();
        p
    }

    /// Configure the pin to a new mode.
    fn mode<const NEWM: Mode>(&mut self) {
        // Configure funcsel? Or should we have done that already?

        gpio_cfg(|| {
            // padreg

            // cfgreg

            // altpad
        })
    }

    pub fn pin_num() -> usize {
        P
    }

    // pub fn into_push_pull_output(self) -> Pin<P, { Mode::Output }> {
    //     // TODO: Configure as output
    //     Pin {}
    // }
}

impl<const P: usize> Pin<P, { Mode::Output }>
where
    Pin<P, { Mode::Output }>: PinCfg,
{
    /// Enable the internal pull up on the pin.
    pub fn internal_pull_up(&mut self, on: bool) {
        self.padpull(on)
    }

    pub fn set_drive_strength(&mut self, d: DriveStrength) {}
}

pub struct Pins {
    _gpio: GPIO,
    pub d13: P5<{ Mode::Floating }>,
}

impl Pins {
    pub fn new(gpio: GPIO) -> Pins {
        // Takes ownership of GPIO.
        Pins {
            _gpio: gpio,
            d13: Pin::new(),
        }
    }
}

// Declare all the pins
pin!(4, B);
pin!(5, B);
pin!(6, B);
pin!(7, B);

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn set_up_led() {
//         let p = pac::Peripherals::take().unwrap();
//     }
// }
