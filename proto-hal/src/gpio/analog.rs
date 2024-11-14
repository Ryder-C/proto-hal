//! Analog specific traits and structures.

/// Type-state for a pin configured in analog mode.
pub struct Analog;

impl super::PinMode for Analog {}

/// Analog pin types implement this trait to represent an analog pin.
pub trait AnalogPin: super::Pin<Analog> {}
