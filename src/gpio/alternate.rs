//! Alternate Function specific traits and structures.

use core::marker::PhantomData;

/// Types implement this trait to be a type-state for an alternate function pin's mode.
pub trait AlternateMode {
    type Word;
    const RAW: Self::Word;
}

/// Type-state for a pin configured in an alternate function mode.
pub struct Alternate<Mode: AlternateMode> {
    _mode: PhantomData<Mode>,
}

impl<Mode: AlternateMode> super::PinMode for Alternate<Mode> {}

/// Analog pin types implement this trait to represent an analog pin.
pub trait AlternatePin<Mode: AlternateMode>: super::Pin<Alternate<Mode>> {}
