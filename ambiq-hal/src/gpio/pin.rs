use super::gpio_cfg;
use crate::hal::digital::v2::{InputPin, IoPin, OutputPin, ToggleableOutputPin};
use core::convert::Infallible;
use core::marker::ConstParamTy;
use embedded_hal::digital::v2::PinState;
use paste::paste;

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

#[derive(PartialEq, Eq, ConstParamTy)]
pub enum Mode {
    Floating,
    Input,
    Output,
    InputOutput,
    AF0,
}

pub struct Pin<const PINNUM: usize, const MODE: Mode> {}

pub trait PinCfg {
    unsafe fn padfncsel(&mut self, f: u8);
    unsafe fn padinpen(&mut self, on: bool);
    unsafe fn padpull(&mut self, on: bool);
    unsafe fn padstrng(&mut self, high: bool);

    unsafe fn outcfg(&mut self, c: u8);
}

macro_rules! pin {
    ($id:literal, $padreg:ident, $cfgreg: ident) => {
        paste! {
            pub type [<P $id>]<const MODE: Mode> = Pin<$id, MODE>;

            impl<const MODE: Mode>PinCfg for Pin<$id, MODE> {
                unsafe fn padpull(&mut self, on: bool) {
                    // XXX: Pad 20 differs in behavior to the rest of the pads, see p. 420.

                    (*pac::GPIO::ptr()).[<padreg $padreg:lower>]
                        .write(|p| p.[< pad $id pull >]().bit(on))
                }

                unsafe fn padinpen(&mut self, on: bool) {
                    (*pac::GPIO::ptr()).[<padreg $padreg:lower>]
                        .write(|p| p.[< pad $id inpen >]().bit(on))
                }

                unsafe fn padstrng(&mut self, high: bool) {
                    (*pac::GPIO::ptr()).[<padreg $padreg:lower>]
                        .write(|p| p.[< pad $id strng >]().bit(high))
                }

                unsafe fn padfncsel(&mut self, f: u8) {
                    (*pac::GPIO::ptr()).[<padreg $padreg:lower>]
                        .write(|p| p.[< pad $id fncsel >]().bits(f))
                }

                unsafe fn outcfg(&mut self, f: u8) {
                    (*pac::GPIO::ptr()).[<cfg $cfgreg:lower>]
                        .write(|p| p.[< gpio $id outcfg >]().bits(f))
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

impl<const P: usize, const M: Mode> Pin<P, M>
where
    Pin<P, M>: PinCfg,
{
    pub fn with_mode() -> Self {
        Self {}.into_mode::<M>()
    }

    /// Configure the pin to a new mode.
    pub fn into_mode<const NEWM: Mode>(mut self) -> Pin<P, NEWM> {
        gpio_cfg(|| unsafe {
            // TODO: Clear the three registers.

            self.padfncsel(3); // 3 is GPIO-mode

            // XXX: Pin 3, 37, 41 can have powersw configured.

            // padreg
            match NEWM {
                Mode::Floating | Mode::Input => {
                    self.padinpen(true);
                    self.outcfg(0);
                }
                Mode::Output => {
                    self.padinpen(false);
                    self.outcfg(1); // push-pull
                }
                Mode::InputOutput => {
                    self.padinpen(true);
                    self.outcfg(1);
                }
                Mode::AF0 => {
                    self.padfncsel(0);
                    self.padinpen(false);
                }
            }
        });

        Pin {}
    }

    pub fn pin_num(&self) -> usize {
        P
    }

    pub fn into_push_pull_output(self) -> Pin<P, { Mode::Output }> {
        self.into_mode()
    }

    pub fn into_input(self) -> Pin<P, { Mode::Input }> {
        self.into_mode()
    }

    pub fn into_input_output(self) -> Pin<P, { Mode::InputOutput }> {
        self.into_mode()
    }
}

impl<const P: usize, const M: Mode> IoPin<Pin<P, { Mode::Input }>, Pin<P, { Mode::Output }>>
    for Pin<P, M>
where
    Pin<P, M>: PinCfg,
    Pin<P, { Mode::Input }>: PinCfg,
    Pin<P, { Mode::Output }>: PinCfg,
{
    type Error = Infallible;

    fn into_input_pin(self) -> Result<Pin<P, { Mode::Input }>, Self::Error> {
        self.into_input_pin()
    }

    fn into_output_pin(self, state: PinState) -> Result<Pin<P, { Mode::Output }>, Self::Error> {
        write_state(P, matches!(state, PinState::High));
        Ok(self.into_push_pull_output())
    }
}

impl<const P: usize> Pin<P, { Mode::Output }>
where
    Pin<P, { Mode::Output }>: PinCfg,
{
    /// Enable the internal pull up on the pin.
    pub fn internal_pull_up(&mut self, on: bool) {
        // XXX: See p. 420 for which pins support this. This should probably be in a
        // trait that is only implemented for those pins.
        gpio_cfg(|| unsafe { self.padpull(on) })
    }
}

impl<const P: usize> Pin<P, { Mode::InputOutput }>
where
    Pin<P, { Mode::InputOutput }>: PinCfg,
{
    /// Enable the internal pull up on the pin.
    pub fn internal_pull_up(&mut self, on: bool) {
        // XXX: See p. 420 for which pins support this. This should probably be in a
        // trait that is only implemented for those pins.
        gpio_cfg(|| unsafe { self.padpull(on) })
    }
}

impl<const P: usize> Pin<P, { Mode::AF0 }>
where
    Pin<P, { Mode::AF0 }>: PinCfg,
{
    pub fn set_drive_strength(&mut self, d: DriveStrength) {
        gpio_cfg(|| unsafe {
            match d {
                DriveStrength::D2mA => self.padstrng(false),
                DriveStrength::D4mA => self.padstrng(true),
                _ => (), // XXX: is configured in altpadcfg
            }
        });
    }
}

impl<const P: usize> OutputPin for Pin<P, { Mode::Output }>
where
    Pin<P, { Mode::Output }>: PinCfg,
{
    type Error = Infallible;

    fn set_low(&mut self) -> Result<(), Infallible> {
        write_state(P, false);
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Infallible> {
        write_state(P, true);
        Ok(())
    }
}

impl<const P: usize> OutputPin for Pin<P, { Mode::InputOutput }>
where
    Pin<P, { Mode::InputOutput }>: PinCfg,
{
    type Error = Infallible;

    fn set_low(&mut self) -> Result<(), Infallible> {
        write_state(P, false);
        defmt::trace!("re-configure for output..");
        let _p = Pin::<P, { Mode::InputOutput }> {}.into_push_pull_output(); // XXX: Re-configure as input
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Infallible> {
        write_state(P, true);
        // defmt::trace!("re-configure for output..");
        // let _p = Pin::<P, { Mode::InputOutput }>{}.into_push_pull_output(); // XXX: Re-configure as input
        Ok(())
    }
}

impl<const P: usize> InputPin for Pin<P, { Mode::Input }>
where
    Pin<P, { Mode::Input }>: PinCfg,
{
    type Error = Infallible;

    fn is_low(&self) -> Result<bool, Self::Error> {
        self.is_high().map(|b| !b)
    }

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(read_input_state(P))
    }
}

impl<const P: usize> InputPin for Pin<P, { Mode::InputOutput }>
where
    Pin<P, { Mode::InputOutput }>: PinCfg,
{
    type Error = Infallible;

    fn is_low(&self) -> Result<bool, Self::Error> {
        self.is_high().map(|b| !b)
    }

    fn is_high(&self) -> Result<bool, Self::Error> {
        // defmt::trace!("re-configure for input..");
        // let _p = Pin::<P, { Mode::InputOutput }>{}.into_input(); // XXX: Re-configure as input
        Ok(read_input_state(P))
    }
}

fn read_input_state(p: usize) -> bool {
    let mask: u32 = 0b1u32 << p % 32;

    let reg = unsafe {
        match p {
            0..=31 => (*pac::GPIO::ptr()).rda.as_ptr(),
            32..=63 => (*pac::GPIO::ptr()).rdb.as_ptr(),
            _ => unimplemented!(),
        }
    };

    defmt::trace!("pin: {}: mask 0b{:b}", p, mask);
    defmt::trace!("pin: {}: register: {}, 0b{:b}", p, reg, unsafe { *reg });

    (unsafe { *reg & mask } != 0)
}

fn write_state(p: usize, val: bool) {
    let mask: u32 = 0b1u32 << p % 32;

    let reg = unsafe {
        match p {
            0..=31 => (*pac::GPIO::ptr()).wtsa.as_ptr(),
            _ => (*pac::GPIO::ptr()).wtsb.as_ptr(),
        }
    };

    gpio_cfg(|| unsafe {
        if val {
            *reg |= mask;
        } else {
            *reg &= !mask;
        }
    });
}

fn toggle_state(P: usize) {
    let mask: u32 = 0b1u32 << P % 32;

    let reg = unsafe {
        match P {
            0..=31 => (*pac::GPIO::ptr()).wta.as_ptr(),
            _ => (*pac::GPIO::ptr()).wtb.as_ptr(),
        }
    };

    gpio_cfg(|| unsafe {
        *reg ^= mask;
    });
}

impl<const P: usize> ToggleableOutputPin for Pin<P, { Mode::Output }>
where
    Pin<P, { Mode::Output }>: PinCfg,
{
    type Error = Infallible;

    fn toggle(&mut self) -> Result<(), Infallible> {
        toggle_state(P);

        Ok(())
    }
}

impl<const P: usize> ToggleableOutputPin for Pin<P, { Mode::InputOutput }>
where
    Pin<P, { Mode::Output }>: PinCfg,
{
    type Error = Infallible;

    fn toggle(&mut self) -> Result<(), Infallible> {
        toggle_state(P);

        Ok(())
    }
}

// Declare the pins
// Pad number, Pad register, Cfg register
pin!(5, B, A);
pin!(6, B, A);
pin!(7, B, A);
pin!(11, C, B);
pin!(13, D, B);
pin!(19, E, C);
pin!(25, G, D);
pin!(27, G, D);
pin!(35, I, E);
pin!(38, J, E);
pin!(39, J, E);
pin!(40, K, F);
pin!(42, K, F);
pin!(43, K, F);
pin!(48, M, G);
pin!(49, M, G);
