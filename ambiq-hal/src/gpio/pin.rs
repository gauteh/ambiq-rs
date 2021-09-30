use pac::GPIO;

#[derive(PartialEq, Eq)]
pub enum DriveStrength {
    D2MA,
    D4MA,
    D8MA,
    D12MA,
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

// TODO: Implement this as a combination of types.
pub struct Pin<const PINNUM: usize, const DRIVESTRENGTH: DriveStrength, const MODE: Mode> {}

impl<const P: usize, const D: DriveStrength, const M: Mode> Pin<P, D, M> {
    pub fn new() -> Self {
        Self {}
    }

    pub fn pin_num() -> usize {
        P
    }

    pub fn into_output_pin(self) -> Pin<P, D, { Mode::Output }> {
        Pin {}
    }
}

impl<const P: usize, const D: DriveStrength> Pin<P, D, { Mode::Output }> {
    /// Enable the internal pull up on the pin.
    pub fn internal_pull_up(&mut self, on: bool) {
        use pac::gpio::padregb::PAD5PULL_A::*;

        unsafe {
            (*pac::GPIO::ptr())
                .padregb
                .write(|p| p.pad5pull().variant(if on { EN } else { DIS }));
        }
    }
}

pub type Pa5<const M: Mode> = Pin<5, { DriveStrength::D12MA }, M>;

pub struct Pins {
    gpio: GPIO,
    d13: Pa5<{ Mode::Floating }>,
}

impl Pins {
    pub fn new(gpio: GPIO) -> Pins {
        Pins {
            gpio,
            d13: Pa5::new(),
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
