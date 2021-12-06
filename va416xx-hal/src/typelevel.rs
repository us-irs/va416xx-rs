//! Module supporting type-level programming
//!
//! This module is identical to the
//! [atsamd typelevel](https://docs.rs/atsamd-hal/latest/atsamd_hal/typelevel/index.html).

use core::ops::{Add, Sub};

use typenum::{Add1, Bit, Sub1, UInt, Unsigned, B1, U0};

mod private {
    /// Super trait used to mark traits with an exhaustive set of
    /// implementations
    pub trait Sealed {}

    impl Sealed for u8 {}
    impl Sealed for i8 {}
    impl Sealed for u16 {}
    impl Sealed for i16 {}
    impl Sealed for u32 {}
    impl Sealed for i32 {}
    impl Sealed for f32 {}

    /// Mapping from an instance of a countable type to its successor
    pub trait Increment {
        /// Successor type of `Self`
        type Inc;
        /// Consume an instance of `Self` and return its successor
        fn inc(self) -> Self::Inc;
    }

    /// Mapping from an instance of a countable type to its predecessor
    pub trait Decrement {
        /// Predecessor type of `Self`
        type Dec;
        /// Consume an instance of `Self` and return its predecessor
        fn dec(self) -> Self::Dec;
    }
}

pub(crate) use private::Decrement as PrivateDecrement;
pub(crate) use private::Increment as PrivateIncrement;
pub(crate) use private::Sealed;

/// Type-level version of the [`None`] variant
#[derive(Default)]
pub struct NoneT;

impl Sealed for NoneT {}

//==============================================================================
// Is
//==============================================================================

/// Marker trait for type identity
///
/// This trait is used as part of the [`AnyKind`] trait pattern. It represents
/// the concept of type identity, because all implementors have
/// `<Self as Is>::Type == Self`. When used as a trait bound with a specific
/// type, it guarantees that the corresponding type parameter is exactly the
/// specific type. Stated differently, it guarantees that `T == Specific` in
/// the following example.
///
/// ```ignore
/// where T: Is<Type = Specific>
/// ```
///
/// Moreover, the super traits guarantee that any instance of or reference to a
/// type `T` can be converted into the `Specific` type.
///
/// ```ignore
/// fn example<T>(mut any: T)
/// where
///     T: Is<Type = Specific>,
/// {
///     let specific_mut: &mut Specific = any.as_mut();
///     let specific_ref: &Specific = any.as_ref();
///     let specific: Specific = any.into();
/// }
/// ```
///
/// [`AnyKind`]: #anykind-trait-pattern
pub trait Is
where
    Self: Sealed,
    Self: From<IsType<Self>>,
    Self: Into<IsType<Self>>,
    Self: AsRef<IsType<Self>>,
    Self: AsMut<IsType<Self>>,
{
    type Type;
}

/// Type alias for [`Is::Type`]
pub type IsType<T> = <T as Is>::Type;

impl<T> Is for T
where
    T: Sealed + AsRef<T> + AsMut<T>,
{
    type Type = T;
}

//==============================================================================
// Counting
//==============================================================================

/// Implement `Sealed` for [`U0`]
impl Sealed for U0 {}

/// Implement `Sealed` for all type-level, [`Unsigned`] integers *except* [`U0`]
impl<U: Unsigned, B: Bit> Sealed for UInt<U, B> {}

/// Trait mapping each countable type to its successor
///
/// This trait maps each countable type to its corresponding successor type. The
/// actual implementation of this trait is contained within `PrivateIncrement`.
/// Access to `PrivateIncrement` is restricted, so that safe HAL APIs can be
/// built with it.
pub trait Increment: PrivateIncrement {}

impl<T: PrivateIncrement> Increment for T {}

/// Trait mapping each countable type to its predecessor
///
/// This trait maps each countable type to its corresponding predecessor type.
/// The actual implementation of this trait is contained within
/// `PrivateDecrement`. Access to `PrivateDecrement` is restricted, so that safe
/// HAL APIs can be built with it.
pub trait Decrement: PrivateDecrement {}

impl<T: PrivateDecrement> Decrement for T {}

impl<N> PrivateIncrement for N
where
    N: Unsigned + Add<B1>,
    Add1<N>: Unsigned,
{
    type Inc = Add1<N>;
    #[inline]
    fn inc(self) -> Self::Inc {
        Self::Inc::default()
    }
}

impl<N> PrivateDecrement for N
where
    N: Unsigned + Sub<B1>,
    Sub1<N>: Unsigned,
{
    type Dec = Sub1<N>;
    #[inline]
    fn dec(self) -> Self::Dec {
        Self::Dec::default()
    }
}
