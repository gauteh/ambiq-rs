use paste::paste;
use pac::GPIO;

// The GPIO_PADKEY register must be set to 0x73 before writing to the PADREGn registers,
// and should be cleared (or set to another value) afterwards.
const PAD_KEY: u32 = 0x73;

pub(crate) fn gpio_cfg<F, R>(f: F) -> R
    where F: FnOnce() -> R
{
    cortex_m::interrupt::free(|_| {
        // page 419
        unsafe {
            (*pac::GPIO::ptr()).padkey.write(|w| w.bits(PAD_KEY) );
        }

        let r = f();

        unsafe {
            (*pac::GPIO::ptr()).padkey.write(|w| w.bits(0x00) );
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

/// GPIO Configuration Register A (Pads 0-7)
/// GPIO Configuration Register B (Pads 8-15)
/// GPIO Configuration Register C (Pads 16-23)
/// GPIO Configuration Register D (Pads 24-31)
/// GPIO Configuration Register E (Pads 32-39)
/// GPIO Configuration Register F (Pads 40 -47)
/// GPIO Configuration Register G (Pads 48-49)
#[derive(PartialEq, Eq)]
pub enum Pad {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Pad {
    pub const fn repr(&self) -> char {
        use Pad::*;

        match self {
            A => 'a',
            B => 'b',
            _ => 'c'
        }
    }

}

// TODO: Implement this as a combination of types.
pub struct Pin<const PINNUM: usize, const MODE: Mode> {}

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

    pub const fn pad() -> Pad {
        match P {
            0..=3 => Pad::A,
            4..=7 => Pad::B,
            _ => Pad::A
        }
    }

    pub fn into_push_pull_output(self) -> Pin<P, { Mode::Output }> {
        // TODO: Configure as output
        Pin {}
    }
}

macro_rules! padreg {
    ($name: literal) => {
        &paste! {
            (*pac::GPIO::ptr()).[<padreg $name>]
        }
    }
}

impl<const P: usize> Pin<P, { Mode::Output }> {
    /// Enable the internal pull up on the pin.
    pub fn internal_pull_up(&mut self, on: bool) {
        use pac::gpio::padregb::PAD5PULL_A::*;

        // 1) Set PAD_KEY
        // 2) Write PADREG
        // 3) Write CFG
        // 4) Write AltPad
        // 5) Clear PAD_KEY

        // XXX: Pad 20 differs in behavior to the rest of the pads, see p. 420.

        gpio_cfg(|| unsafe {
            // let gpio = &(*pac::GPIO::ptr());
            // let padreg = &paste! { gpio.[<padreg b>] };
            padreg!("b").write(|p| p.pad5pull().variant(if on { EN } else { DIS }));
        })
    }

    pub fn set_drive_strength(&mut self, d: DriveStrength) {
    }
}

pub struct Pins {
    gpio: GPIO,
    pub d13: Pin<5, { Mode::Floating }>,
}

impl Pins {
    pub fn new(gpio: GPIO) -> Pins {
        // Takes ownership of GPIO.
        Pins {
            gpio,
            d13: Pin::new(),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn set_up_led() {
//         let p = pac::Peripherals::take().unwrap();
//     }
// }
