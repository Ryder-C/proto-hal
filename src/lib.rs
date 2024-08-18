#![no_std]

pub mod gpio;

#[cfg(feature = "stm32")]
pub mod stm32;

/// Types that encapsulate a resource that can be configured to be
/// in an "inactive" state implement this trait.
pub trait IntoInactive: Into<Self::Inactive> {
    /// The type-state representing an "inactive" mode.
    type Inactive;
}

/// Types that encapsulate a resource with events that can be "listened"
/// for (i.e. related interrupts) implement this trait.
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

/// Types that encapsulate a resource implement this trait
/// to release the resource.
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

/// Types implement this trait to be validated.
///
/// *Note: Implementors should provide concrete const implementations
/// when possible.*
pub trait Validate {
    type Validated;
    type Error;

    /// Consume the value and attempt to validate it's contents.
    fn validate(self) -> Result<Self::Validated, Self::Error>
    where
        Self: Sized;
}
