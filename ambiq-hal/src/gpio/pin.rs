use pac::GPIO;

pub enum DriveStrength {
    D2MA,
    D4MA,
    D8MA,
    D12MA
}

pub struct Pin {
    pinnum: u32,
    drivestrength: DriveStrength,
}

pub struct Pins {
    led: Pin,
}

impl Pins {
    // pub fn new(gpio: GPIO) -> Pins {
    // }
}

const PA5: Pin = Pin { pinnum: 5, drivestrength: DriveStrength::D12MA };

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn set_up_led() {
//         let p = pac::Peripherals::take().unwrap();
//     }
// }
