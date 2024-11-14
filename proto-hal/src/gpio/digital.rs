//! Digital specific traits and structures.

use core::{fmt::Debug, marker::PhantomData};

/// Represents the possible levels of a digital pin's value.
#[derive(Debug, PartialEq)]
pub enum Level {
    /// Logic level "low".
    Low,
    /// Logic level "high".
    High,
}

/// Types implement this trait to be a type-state for a digital input pin's mode.
pub trait InputMode {}
/// Types implement this trait to be a type-state for a digital output pin's mode.
pub trait OutputMode {}

/// Type-state for a pin configured in input mode.
pub struct Input<Mode: InputMode> {
    _mode: PhantomData<Mode>,
}

/// Type-state for a pin configured in output mode.
pub struct Output<Mode: OutputMode> {
    _mode: PhantomData<Mode>,
}

impl<Mode: InputMode> super::PinMode for Input<Mode> {}
impl<Mode: OutputMode> super::PinMode for Output<Mode> {}

/// Input pin types implement this trait to represent a digital input.
pub trait InputPin<Mode: InputMode>: super::Pin<Input<Mode>> {
    /// Encapsulates the error variants that can occur
    /// when conducting digital input.
    type Error: Debug;

    /// Get the measured level of this pin.
    fn input_level(&self) -> Result<Level, Self::Error>;

    /// Determine whether the measured level on this pin is `Low`.
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(match self.input_level()? {
            Level::Low => true,
            Level::High => false,
        })
    }

    /// Determine whether the measured level on this pin is `High`.
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(!self.is_low()?)
    }
}

/// Output pin types implement this trait to represent a digital output.
pub trait OutputPin<Mode: OutputMode>: super::Pin<Output<Mode>> {
    /// Encapsulates the error variants that can occur
    /// when conducting digital output.
    type Error;

    /// Output a level on this pin.
    fn set_level(&mut self, level: Level) -> Result<(), Self::Error> {
        match level {
            Level::Low => self.set_low(),
            Level::High => self.set_high(),
        }
    }

    /// Output `Low` on this pin.
    fn set_low(&mut self) -> Result<(), Self::Error>;

    /// Output `High` on this pin.
    fn set_high(&mut self) -> Result<(), Self::Error>;
}

/// Digital output pin types implement this trait to represent a stateful digital output.
pub trait StatefulOutputPin<Mode: OutputMode>: OutputPin<Mode> {
    /// Encapsulates the error variants that can occur
    /// when conducting stateful digital output
    /// (in addition to stateless digital output).
    type Error: From<<Self as OutputPin<Mode>>::Error>;

    /// Get the currently outputted level on this pin.
    fn output_level(&self) -> Result<Level, <Self as StatefulOutputPin<Mode>>::Error>;

    /// Determine whether the outputted level on this pin is `Low`.
    fn is_set_low(&self) -> Result<bool, <Self as StatefulOutputPin<Mode>>::Error> {
        Ok(match self.output_level()? {
            Level::Low => true,
            Level::High => false,
        })
    }

    /// Determine whether the outputted level on this pin is `High`.
    fn is_set_high(&self) -> Result<bool, <Self as StatefulOutputPin<Mode>>::Error> {
        Ok(!self.is_set_low()?)
    }

    /// Toggle the output level of this pin.
    fn toggle(&mut self) -> Result<(), <Self as StatefulOutputPin<Mode>>::Error> {
        match self.output_level()? {
            Level::Low => {
                self.set_high()?;
            }
            Level::High => {
                self.set_low()?;
            }
        }

        Ok(())
    }
}
