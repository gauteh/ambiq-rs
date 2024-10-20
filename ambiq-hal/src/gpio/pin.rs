use super::gpio_cfg;
use crate::hal::digital::v2::{InputPin, IoPin, OutputPin, ToggleableOutputPin};
use core::convert::Infallible;
use core::marker::ConstParamTy;
use embedded_hal::digital::v2::PinState;
use paste::paste;

/// Interrupt trigger options.
///
/// Table 603, p. 426.
#[derive(PartialEq, Eq)]
pub enum InterruptOpt {
    LowToHigh,
    HighToLow,
    Disabled,
    Any,
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

    unsafe fn incfg(&mut self, f: bool);
    unsafe fn intd(&mut self, f: bool);
    unsafe fn inten(&mut self, f: bool);
    unsafe fn intstat(&self) -> bool;
    unsafe fn intset(&mut self, f: bool);
    unsafe fn intclr(&mut self, f: bool);
}

macro_rules! pin {
    ($id:literal, $padreg:ident, $cfgreg: ident, $intreg:literal) => {
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

                /// Interrupt cfg.
                unsafe fn incfg(&mut self, f: bool) {
                    (*pac::GPIO::ptr()).[<cfg $cfgreg:lower>]
                        .write(|p| p.[< gpio $id incfg >]().bit(f))
                }

                /// Interrupt cfg.
                unsafe fn intd(&mut self, f: bool) {
                    (*pac::GPIO::ptr()).[<cfg $cfgreg:lower>]
                        .write(|p| p.[< gpio $id intd >]().bit(f))
                }

                unsafe fn inten(&mut self, f: bool) {
                    (*pac::GPIO::ptr()).[< int $intreg en >].write(|p| p.[< gpio $id >]().bit(f))
                }

                unsafe fn intset(&mut self, f: bool) {
                    (*pac::GPIO::ptr()).[< int $intreg set >].write(|p| p.[< gpio $id >]().bit(f))
                }

                unsafe fn intclr(&mut self, f: bool) {
                    (*pac::GPIO::ptr()).[< int $intreg clr >].write(|p| p.[< gpio $id >]().bit(f))
                }

                unsafe fn intstat(&self) -> bool {
                    (*pac::GPIO::ptr()).[< int $intreg stat >].read().[< gpio $id >]().bit_is_set()
                }
            }
        }
    };
}

/// Enable GPIO interrupts globally, they must now be configured for indvidual pads.
pub fn enable_gpio_interrupts() {
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::GPIO);
    }
}

/// Disable GPIO interrupts globally.
pub fn disable_gpio_interrupts() {
    pac::NVIC::mask(pac::Interrupt::GPIO);
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

    pub fn into_push_pull_output_val(self, high: bool) -> Pin<P, { Mode::Output }> {
        set_state(P);
        self.into_mode()
    }

    pub fn into_input(self) -> Pin<P, { Mode::Input }> {
        self.into_mode()
    }

    pub fn into_input_output(self) -> Pin<P, { Mode::InputOutput }> {
        self.into_mode()
    }

    /// Check whether this pin generated the interrupt.
    pub fn interrupt_status(&self) -> bool {
        unsafe { self.intstat() }
    }

    /// Enable interrupts for this pin. Remember to [enable gpio interrupts globally](`enable_gpio_interrupts`) also [configure it](`Pin::configure_interrupt`).
    pub fn enable_interrupt(&mut self) {
        gpio_cfg(|| unsafe {
            self.inten(true);
        });
    }

    /// Configure the interrupt trigger.
    pub fn configure_interrupt(&mut self, dir: InterruptOpt) {
        use InterruptOpt::*;

        gpio_cfg(|| unsafe {
            match dir {
                LowToHigh => {
                    self.incfg(false);
                    self.intd(false);
                }
                HighToLow => {
                    self.incfg(false);
                    self.intd(true);
                }
                Disabled => {
                    self.incfg(true);
                    self.intd(false);
                }
                Any => {
                    self.incfg(true);
                    self.intd(true);
                }
            }
        });
    }

    /// Disable interrupts for this pin.
    pub fn disable_interrupt(&mut self) {
        gpio_cfg(|| unsafe {
            self.inten(false);
        });
    }

    /// Clear the interrupt status for this pin.
    pub fn clear_interrupt(&mut self) {
        gpio_cfg(|| unsafe {
            self.intclr(true);
        });
    }

    /// Instantly generate an interrupt (usually for testing purposes).
    pub fn set_interrupt(&mut self) {
        gpio_cfg(|| unsafe {
            self.intset(true);
        });
    }
}

/// Interrupt register are ordered from highest pad no to lowest: 31 -> 0.
///
/// pad 31 -> bit offset 0
const fn interrupt_mask(gpio: usize) -> u32 {
    0b1u32 << (31 - gpio) % 32
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
        Ok(self.into_input())
    }

    fn into_output_pin(self, state: PinState) -> Result<Pin<P, { Mode::Output }>, Self::Error> {
        match state {
            PinState::High => set_state(P),
            PinState::Low => clear_state(P),
        };
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

    pub fn open_drain(&mut self) {
        gpio_cfg(|| unsafe {
            if P == 40 {
                // self.padfncsel(4);
            }
            self.outcfg(2);
        });
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
        clear_state(P);
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Infallible> {
        set_state(P);
        Ok(())
    }
}

impl<const P: usize> OutputPin for Pin<P, { Mode::InputOutput }>
where
    Pin<P, { Mode::InputOutput }>: PinCfg,
{
    type Error = Infallible;

    fn set_low(&mut self) -> Result<(), Infallible> {
        clear_state(P);
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Infallible> {
        set_state(P);
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

fn set_state(p: usize) {
    let mask: u32 = 0b1u32 << p % 32;

    let reg = unsafe {
        match p {
            0..=31 => (*pac::GPIO::ptr()).wtsa.as_ptr(),
            _ => (*pac::GPIO::ptr()).wtsb.as_ptr(),
        }
    };

    gpio_cfg(|| unsafe {
        *reg |= mask;
    });
}

fn clear_state(p: usize) {
    let mask: u32 = 0b1u32 << p % 32;

    let reg = unsafe {
        match p {
            0..=31 => (*pac::GPIO::ptr()).wtca.as_ptr(),
            _ => (*pac::GPIO::ptr()).wtcb.as_ptr(),
        }
    };

    gpio_cfg(|| unsafe {
        *reg |= mask;
    });
}

fn toggle_state(p: usize) {
    let mask: u32 = 0b1u32 << p % 32;

    let reg = unsafe {
        match p {
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
pin!(5, B, A, 0);
pin!(6, B, A, 0);
pin!(7, B, A, 0);
pin!(11, C, B, 0);
pin!(12, D, B, 0);
pin!(13, D, B, 0);
pin!(19, E, C, 0);
pin!(25, G, D, 0);
pin!(27, G, D, 0);
pin!(29, H, D, 0);
pin!(31, H, D, 0);
pin!(35, I, E, 1);
pin!(38, J, E, 1);
pin!(39, J, E, 1);
pin!(40, K, F, 1);
pin!(42, K, F, 1);
pin!(43, K, F, 1);
pin!(48, M, G, 1);
pin!(49, M, G, 1);
