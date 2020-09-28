use base::cell::{Cell, RefCell, UnsafeCell};
use base::cmp::Reverse;
use base::mem::ManuallyDrop;
use base::num::Wrapping;

#[cfg(feature = "unstable")]
use maybe_std::{pin::Pin, ops::Deref};

use crate::Wrapper;

implement_into_inner_t!(Cell<T>);
implement_into_inner_t!(RefCell<T>);
implement_into_inner_t!(UnsafeCell<T>);

implement_newtype_t!(Reverse<T>);

implement_into_inner_t!(ManuallyDrop<T>);

macro_rules! implement_nonzero {
    ($t:ty, $from:ty) => {
        impl Wrapper<$from> for $t {
            fn into_inner(self) -> $from {
                self.get()
            }
        }
    }
}

implement_nonzero!(core::num::NonZeroU8, u8);
implement_nonzero!(core::num::NonZeroU16, u16);
implement_nonzero!(core::num::NonZeroU32, u32);
implement_nonzero!(core::num::NonZeroU64, u64);
implement_nonzero!(core::num::NonZeroU128, u128);
implement_nonzero!(core::num::NonZeroUsize, usize);
implement_nonzero!(core::num::NonZeroI8, i8);
implement_nonzero!(core::num::NonZeroI16, i16);
implement_nonzero!(core::num::NonZeroI32, i32);
implement_nonzero!(core::num::NonZeroI64, i64);
implement_nonzero!(core::num::NonZeroI128, i128);
implement_nonzero!(core::num::NonZeroIsize, isize);

implement_newtype_t!(Wrapping<T>);

#[cfg(feature = "unstable")]
impl<T: Deref<Target: Unpin>> Wrapper<T> for Pin<T> {
    fn into_inner(self) -> T {
        Pin::into_inner(self)
    }
}

implement_into_inner!(core::sync::atomic::AtomicBool, bool);
implement_into_inner!(core::sync::atomic::AtomicI8, i8);
implement_into_inner!(core::sync::atomic::AtomicI16, i16);
implement_into_inner!(core::sync::atomic::AtomicI32, i32);
implement_into_inner!(core::sync::atomic::AtomicI64, i64);
implement_into_inner!(core::sync::atomic::AtomicIsize, isize);
implement_into_inner!(core::sync::atomic::AtomicU8, u8);
implement_into_inner!(core::sync::atomic::AtomicU16, u16);
implement_into_inner!(core::sync::atomic::AtomicU32, u32);
implement_into_inner!(core::sync::atomic::AtomicU64, u64);
implement_into_inner!(core::sync::atomic::AtomicUsize, usize);
