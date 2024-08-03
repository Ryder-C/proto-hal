pub trait WakeupPin {
    type WakeUp;
    const LANE: Self::WakeUp;
}
