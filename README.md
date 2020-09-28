# Wrapper

A `Wrapper` is a value which took ownership of some of the value, such that the wrapped value can later be retrieved again. Conceptually this is identical to the regular `Into` trait, but that trait is hard to implement because of conflicts with the reflexive blanket implementation.

```rust
/// A type that wraps a value of type `Inner` that can be retrieved via `into_inner`.
pub trait Wrapper<Inner> {
    /// Retrieve ownership of the wrapped value.
    fn into_inner(self) -> Inner;
}
```

## Feature Flags

By default, this crate only provides implementations for types in `core`. The `alloc` feature enables implementations for types in `alloc`, likewise `std` for `std`. Implementations for unstable types are enabled with the `unstable` feature.
