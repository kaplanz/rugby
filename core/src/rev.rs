//! Revision markers.

use std::fmt::{Debug, Display};

pub(crate) mod sealed {
    pub(crate) trait Sealed {}
}

/// Hardware revision marker.
#[expect(private_bounds)]
pub trait Revision: sealed::Sealed + Debug + Display + Default + Send + Sync + 'static {}

impl<T: sealed::Sealed + Debug + Display + Default + Send + Sync + 'static> Revision for T {}
