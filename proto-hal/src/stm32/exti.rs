pub enum SignalEdge {
    Rising,
    Falling,
    Any,
}

pub trait ExtiExt {
    type Channel;

    fn wakeup(&self, event: Self::Channel);
    fn listen(&self, event: Self::Channel, edge: SignalEdge);
    fn unlisten(&self, event: Self::Channel);
    fn is_pending(&self, event: Self::Channel, edge: SignalEdge) -> bool;
    fn unpend(&self, event: Self::Channel);
}

pub trait ExtiPin {
    type Channel;
    type Output;

    const CHANNEL: Self::Channel;

    /// Configures the pin as external trigger
    fn listen(self, edge: SignalEdge, exti: &mut impl ExtiExt) -> Self::Output;
    /// Get the corresponding EXTI Channel variant for this pin.
    fn channel(&self) -> Self::Channel {
        Self::CHANNEL
    }
}
