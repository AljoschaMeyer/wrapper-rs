//! A `Wrapper` is a value which took ownership of some of the value, such that the wrapped value
//! can later be retrieved again. Conceptually this is identical to the regular `Into` trait, but
//! that trait is hard to implement because of conflicts with the reflexive blanket implementation.
//!
//! ## Feature Flags
//!
//! By default, this crate only provides implementations for types in `core`. The `alloc` feature
//! enables implementations for types in `alloc`, likewise `std` for `std`. Implementations for
//! unstable types are enabled with the `unstable` feature.

#![no_std]
#![cfg_attr(feature = "unstable", feature(associated_type_bounds))]
extern crate maybe_std as base;

/// A type that wraps a value of type `Inner` that can be retrieved via `into_inner`.
pub trait Wrapper<Inner> {
    /// Retrieve ownership of the wrapped value.
    fn into_inner(self) -> Inner;
}

// Internal macros for quickly defining implementations.

macro_rules! implement_newtype_t {
    ($t:ty) => {
        impl<T> Wrapper<T> for $t {
            fn into_inner(self) -> T {
                self.0
            }
        }
    }
}

macro_rules! implement_into_inner {
    ($t:ty, $from:ty) => {
        impl Wrapper<$from> for $t {
            fn into_inner(self) -> $from {
                <$t>::into_inner(self)
            }
        }
    }
}

macro_rules! implement_into_inner_t {
    ($t:ty) => {
        impl<T> Wrapper<T> for $t {
            fn into_inner(self) -> T {
                <$t>::into_inner(self)
            }
        }
    }
}

mod impls_core;
pub use impls_core::*;

#[cfg(any(feature = "alloc", feature = "std"))]
mod impls_alloc;
#[cfg(any(feature = "alloc", feature = "std"))]
pub use impls_alloc::*;

#[cfg(feature = "std")]
mod impls_std;
#[cfg(feature = "std")]
pub use impls_std::*;
