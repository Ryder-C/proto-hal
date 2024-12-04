use core::marker::PhantomData;

/// A trait providing an interface to freeze stateful types.
pub trait Freeze: Sized {
    fn freeze<const N: usize>(self) -> (Frozen<Self, N>, [Entitlement<Self>; N]) {
        (
            Frozen { p: self },
            core::array::from_fn(|_| Entitlement { _p: PhantomData }), // this may introduce overhead, will have to investigate (seems not to)
        )
    }
}

impl<T> Freeze for T where T: Sized {}

/// A struct to hold stateful types where
/// the state is frozen.
pub struct Frozen<P, const OBSERVERS: usize>
where
    P: Freeze,
{
    p: P,
}

impl<P, const OBSERVERS: usize> Frozen<P, OBSERVERS>
where
    P: Freeze,
{
    pub fn inner(&self) -> &P {
        &self.p
    }

    pub fn inner_mut(&mut self) -> &mut P {
        &mut self.p
    }
}

/// Indicates a type-state is
/// entitled to another type-state.
pub unsafe trait Entitled<State> {}

/// A struct to represent an entitlement
/// to a type frozen in a particular state.
pub struct Entitlement<P>
where
    P: Freeze,
{
    _p: PhantomData<P>,
}

impl<P> From<P> for Entitlement<P>
where
    P: Freeze,
{
    fn from(_: P) -> Self {
        Self { _p: PhantomData }
    }
}

/// A marker type for
/// an unsatisfied entitlement.
pub struct Unsatisfied;
