pub trait AdcPin<ADC> {
    type ID;
    const CHANNEL: Self::ID;
}
