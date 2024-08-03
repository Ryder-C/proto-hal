#![no_std]

pub mod stm32;

pub trait IntoInactive: Into<Self::Inactive> {
    /// The type-state representing an "inactive" mode.
    type Inactive;
}

pub trait Listen {
    /// The events available to listen for.
    type Events;

    /// Register an event to be listened for.
    fn listen(&mut self, event: Self::Events);

    /// Unregister an even to be listened for.
    fn unlisten(&mut self, event: Self::Events);

    /// Manually pend an event.
    fn pend(&mut self, event: Self::Events);

    /// Manually unpend an event.
    fn unpend(&mut self, event: Self::Events);
}

pub trait Release {
    /// The enclosed resource to be released.
    type Resource: IntoInactive;

    /// Release the underlying resource as a noop.
    ///
    /// # Safety
    ///
    /// The underlying hardware may remain active.
    /// Refer to `disable_and_release` for a safe
    /// implementation.
    unsafe fn release(self) -> Self::Resource;

    /// Release the underlying resource after disabling it.
    ///
    /// Refer to `release` for an unsafe noop implementation.
    fn disable_and_release(self) -> <Self::Resource as IntoInactive>::Inactive;
}

/// GPIO related traits and structures.
pub mod gpio {
    /// Types implement this triat to be a type-state for a pin's mode.
    pub trait PinMode {}
    /// Types implement this trait to be a type-state for an input pin's mode.
    pub trait InputMode: PinMode {}
    /// Types implement this trait to be a type-state for an output pin's mode.
    pub trait OutputMode: PinMode {}

    /// Types implement this trait to represent a GPIO pin.
    pub trait Pin<Mode: PinMode> {}
    /// Types implement this trait to represent a GPIO pin in an input mode.
    pub trait InputPin {}
    /// Types implement this trait to represent a GPIO pin in an output mode.
    pub trait OutputPin {}

    /// Pin types implement this trait to convert into another mode.
    pub trait IntoMode<Mode: PinMode>: Pin<Mode> + Into<Self::Into> {
        /// The type representing the pin in the new mode.
        type Into;

        /// Put the pin into the target mode.
        fn into_mode(self) -> Self::Into;
    }

    /// Pin types implement this trait to convert into an input mode.
    pub trait IntoInput<Mode>: IntoMode<Mode>
    where
        Mode: InputMode,
    {
        /// Put the pin into an input mode.
        fn into_input(self) -> Self::Into {
            self.into_mode()
        }
    }

    /// Pin types implement this trait to convert into an output mode.
    pub trait IntoOutput<Mode>: IntoMode<Mode>
    where
        Mode: OutputMode,
    {
        /// Put the pin into an output mode.
        fn into_output(self) -> Self::Into {
            self.into_mode()
        }
    }

    /// Digital specific traits and structures.
    pub mod digital {
        /// Represents the possible levels of a digital pin's value.
        pub enum Level {
            /// Logic level "low".
            Low,
            /// Logic level "high".
            High,
        }

        /// Input pin types implement this trait to represent a digital input.
        pub trait InputPin: super::InputPin {
            /// Encapsulates the error variants that can occur
            /// when conducting digital input.
            type Error;

            /// Get the measured level of this pin.
            fn level(&self) -> Result<Level, Self::Error>;

            /// Determine whether the measured level on this pin is `Low`.
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(match self.level()? {
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
        pub trait OutputPin: super::OutputPin {
            /// Encapsulates the error variants that can occur
            /// when conducting digital output.
            type Error;

            /// Output a level on this pin.
            fn set_level(&mut self, level: Level) -> Result<(), Self::Error>;

            /// Output `Low` on this pin.
            fn set_low(&mut self) -> Result<(), Self::Error> {
                self.set_level(Level::Low)
            }

            /// Output `High` on this pin.
            fn set_high(&mut self) -> Result<(), Self::Error> {
                self.set_level(Level::High)
            }
        }

        /// Digital output pin types implement this trait to represent a stateful digital output.
        pub trait StatefulOutputPin: OutputPin {
            /// Encapsulates the error variants that can occur
            /// when conducting stateful digital output
            /// (in addition to stateless digital output).
            type Error: From<<Self as OutputPin>::Error>;

            /// Get the currently outputted level on this pin.
            fn level(&self) -> Result<Level, <Self as StatefulOutputPin>::Error>;

            /// Determine whether the outputted level on this pin is `Low`.
            fn is_set_low(&self) -> Result<bool, <Self as StatefulOutputPin>::Error> {
                Ok(match self.level()? {
                    Level::Low => true,
                    Level::High => false,
                })
            }

            /// Determine whether the outputted level on this pin is `High`.
            fn is_set_high(&self) -> Result<bool, <Self as StatefulOutputPin>::Error> {
                Ok(!self.is_set_low()?)
            }

            /// Toggle the output level of this pin.
            fn toggle(&mut self) -> Result<(), <Self as StatefulOutputPin>::Error> {
                match self.level()? {
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
    }
}
