use base::io::{BufReader, Cursor, Take};

use crate::Wrapper;

implement_into_inner_t!(BufReader<T>);
implement_into_inner_t!(Cursor<T>);
implement_into_inner_t!(Take<T>);
