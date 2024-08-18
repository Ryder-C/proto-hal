//! GPIO related traits and structures.

#[cfg(feature = "pin_alternates")]
pub mod alternate;
pub mod analog;
pub mod digital;

/// Types implement this triat to be a type-state for a pin's mode.
pub trait PinMode {}

/// Types implement this trait to represent a GPIO pin.
pub trait Pin<Mode: PinMode> {}

/// Pin types implement this trait to convert into another mode.
pub trait IntoMode<Mode: PinMode> {
    /// The type representing the pin in the new mode.
    type Into: Pin<Mode>;

    /// Put the pin into the target mode.
    fn into_mode(self) -> Self::Into;
}

#[cfg(test)]
mod tests {
    use core::{convert::Infallible, marker::PhantomData};

    use crate::gpio;

    // it is the HAL's responsibility
    // to do the following implementations

    // input modes
    struct Floating;
    struct PullUp;
    struct PullDown;

    // output modes
    struct OpenDrain;
    struct PushPull;

    // alternate modes
    #[cfg(feature = "pin_alternates")]
    struct AF0;

    impl gpio::digital::InputMode for Floating {}
    impl gpio::digital::InputMode for PullUp {}
    impl gpio::digital::InputMode for PullDown {}
    impl gpio::digital::OutputMode for OpenDrain {}
    impl gpio::digital::OutputMode for PushPull {}

    #[cfg(feature = "pin_alternates")]
    impl gpio::alternate::AlternateMode for AF0 {
        type Word = u8;
        const RAW: Self::Word = 0;
    }

    /// Dummy pin.
    struct PA0<Mode: gpio::PinMode> {
        _mode: PhantomData<Mode>,
    }

    // impl pin

    impl<Mode: gpio::PinMode> gpio::Pin<Mode> for PA0<Mode> {}

    // impl analog mode and conversion

    impl gpio::analog::AnalogPin for PA0<gpio::analog::Analog> {}

    impl<Mode: gpio::PinMode> gpio::IntoMode<gpio::analog::Analog> for PA0<Mode> {
        type Into = PA0<gpio::analog::Analog>;

        fn into_mode(self) -> Self::Into {
            PA0 { _mode: PhantomData }
        }
    }

    // impl input modes and conversions

    impl<Mode: gpio::digital::InputMode> gpio::digital::InputPin<Mode>
        for PA0<gpio::digital::Input<Mode>>
    {
        type Error = Infallible;

        fn input_level(&self) -> Result<gpio::digital::Level, Self::Error> {
            Ok(gpio::digital::Level::High)
        }
    }

    impl<Mode: gpio::PinMode> gpio::IntoMode<gpio::digital::Input<Floating>> for PA0<Mode> {
        type Into = PA0<gpio::digital::Input<Floating>>;

        fn into_mode(self) -> Self::Into {
            // no-op
            PA0 { _mode: PhantomData }
        }
    }

    impl<Mode: gpio::PinMode> gpio::IntoMode<gpio::digital::Input<PullUp>> for PA0<Mode> {
        type Into = PA0<gpio::digital::Input<PullUp>>;

        fn into_mode(self) -> Self::Into {
            // no-op
            PA0 { _mode: PhantomData }
        }
    }

    impl<Mode: gpio::PinMode> gpio::IntoMode<gpio::digital::Input<PullDown>> for PA0<Mode> {
        type Into = PA0<gpio::digital::Input<PullDown>>;

        fn into_mode(self) -> Self::Into {
            // no-op
            PA0 { _mode: PhantomData }
        }
    }

    // impl output modes and conersions

    impl<Mode: gpio::PinMode> gpio::IntoMode<gpio::digital::Output<OpenDrain>> for PA0<Mode> {
        type Into = PA0<gpio::digital::Output<OpenDrain>>;

        fn into_mode(self) -> Self::Into {
            // no-op
            PA0 { _mode: PhantomData }
        }
    }

    impl<Mode: gpio::PinMode> gpio::IntoMode<gpio::digital::Output<PushPull>> for PA0<Mode> {
        type Into = PA0<gpio::digital::Output<PushPull>>;

        fn into_mode(self) -> Self::Into {
            // no-op
            PA0 { _mode: PhantomData }
        }
    }

    impl<Mode: gpio::digital::OutputMode> gpio::digital::OutputPin<Mode>
        for PA0<gpio::digital::Output<Mode>>
    {
        type Error = Infallible;

        fn set_low(&mut self) -> Result<(), Self::Error> {
            // no-op
            Ok(())
        }

        fn set_high(&mut self) -> Result<(), Self::Error> {
            // no-op
            Ok(())
        }
    }

    impl<Mode: gpio::digital::OutputMode> gpio::digital::StatefulOutputPin<Mode>
        for PA0<gpio::digital::Output<Mode>>
    {
        type Error = Infallible;

        fn output_level(
            &self,
        ) -> Result<gpio::digital::Level, <Self as gpio::digital::StatefulOutputPin<Mode>>::Error>
        {
            // dummy
            Ok(gpio::digital::Level::High)
        }
    }

    // impl alternate modes and conversions

    #[cfg(feature = "pin_alternates")]
    impl<Mode: gpio::PinMode> gpio::IntoMode<gpio::alternate::Alternate<AF0>> for PA0<Mode> {
        type Into = PA0<gpio::alternate::Alternate<AF0>>;

        fn into_mode(self) -> Self::Into {
            PA0 { _mode: PhantomData }
        }
    }

    // Explicit conversions.
    impl<Mode: gpio::PinMode> PA0<Mode> {
        fn into_input_floating(self) -> PA0<gpio::digital::Input<Floating>> {
            gpio::IntoMode::<gpio::digital::Input<Floating>>::into_mode(self)
        }

        fn into_input_pull_up(self) -> PA0<gpio::digital::Input<PullUp>> {
            gpio::IntoMode::<gpio::digital::Input<PullUp>>::into_mode(self)
        }

        fn into_input_pull_down(self) -> PA0<gpio::digital::Input<PullDown>> {
            gpio::IntoMode::<gpio::digital::Input<PullDown>>::into_mode(self)
        }

        fn into_output_open_drain(self) -> PA0<gpio::digital::Output<OpenDrain>> {
            gpio::IntoMode::<gpio::digital::Output<OpenDrain>>::into_mode(self)
        }

        fn into_output_push_pull(self) -> PA0<gpio::digital::Output<PushPull>> {
            gpio::IntoMode::<gpio::digital::Output<PushPull>>::into_mode(self)
        }

        fn into_analog(self) -> PA0<gpio::analog::Analog> {
            gpio::IntoMode::<gpio::analog::Analog>::into_mode(self)
        }

        #[cfg(feature = "pin_alternates")]
        fn into_alternate_af0(self) -> PA0<gpio::alternate::Alternate<AF0>> {
            gpio::IntoMode::<gpio::alternate::Alternate<AF0>>::into_mode(self)
        }
    }

    #[test]
    fn conversions() {
        let pa0 = PA0::<gpio::analog::Analog> { _mode: PhantomData };
        let input_floating = pa0.into_input_floating();
        let input_pull_up = input_floating.into_input_pull_up();
        let input_pull_down = input_pull_up.into_input_pull_down();
        let output_open_drain = input_pull_down.into_output_open_drain();
        let output_push_pull = output_open_drain.into_output_push_pull();
        #[cfg(feature = "pin_alternates")]
        {
            let af0 = output_push_pull.into_alternate_af0();
            let _analog = af0.into_analog();
        }
        #[cfg(not(feature = "pin_alternates"))]
        let _analog = output_push_pull.into_analog();
    }

    #[test]
    fn trait_usage() {
        let pa0 = PA0::<gpio::analog::Analog> { _mode: PhantomData };

        fn wants_a_pull_up(_pin: &impl gpio::digital::InputPin<PullUp>) {
            // no-op
        }

        fn makes_a_pull_down<Pin>(pin: Pin) -> Pin::Into
        where
            Pin: gpio::IntoMode<gpio::digital::Input<PullDown>>,
        {
            pin.into_mode() // provided by trait, mode inferred
        }

        let input_pull_up = pa0.into_input_pull_up();

        wants_a_pull_up(&input_pull_up);

        let _input_pull_down = makes_a_pull_down(input_pull_up);
    }

    mod digital {
        use super::*;

        use gpio::digital::{InputPin as _, OutputPin as _, StatefulOutputPin as _};

        #[test]
        fn input() {
            let pa0 = PA0::<gpio::analog::Analog> { _mode: PhantomData };

            let input = pa0.into_input_pull_up();

            assert_eq!(input.input_level().unwrap(), gpio::digital::Level::High);
            assert!(!input.is_low().unwrap());
            assert!(input.is_high().unwrap());
        }

        #[test]
        fn output() {
            let pa0 = PA0::<gpio::analog::Analog> { _mode: PhantomData };

            let mut output = pa0.into_output_push_pull();

            assert_eq!(output.output_level().unwrap(), gpio::digital::Level::High);
            assert!(!output.is_set_low().unwrap());
            assert!(output.is_set_high().unwrap());
            output.set_level(gpio::digital::Level::Low).unwrap();
            output.set_low().unwrap();
            output.set_high().unwrap();
        }
    }
}
