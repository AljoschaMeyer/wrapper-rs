use base::prelude::v1::*;
use base::borrow::Cow;

use crate::Wrapper;

impl<B: ?Sized + ToOwned> Wrapper<<B as ToOwned>::Owned> for Cow<'_, B> {
    fn into_inner(self) -> <B as ToOwned>::Owned {
        self.into_owned()
    }
}

impl<T> Wrapper<T> for Box<T> {
    fn into_inner(self) -> T {
        *self
    }
}
