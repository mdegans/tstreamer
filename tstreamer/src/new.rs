use serde::{de::DeserializeOwned, Serialize};

use crate::backends::Backend;

/// A trait for `Options` that can be used to construct various objects such as
/// [`Element`]s in a [`Pipeline`].
///
/// You should implement [`New`] instead of this trait. There is a blanket
/// implementation for all [`New`] types.
pub trait Options: Serialize + DeserializeOwned + Sized + 'static {
    /// The [`New`] thing to construct.
    type New: New<Options = Self>;

    /// Builds the thing.
    fn build(self, backend: Backend) -> Self::New {
        Self::New::new(self, backend)
    }
}

/// A trait for constructing various objects such as [`Element`]s in a
/// [`Pipeline`].
///
/// [`Pipeline`]: crate::pipeline::Pipeline
pub trait New: Serialize + DeserializeOwned + 'static {
    /// The type of the options.
    type Options: Options<New = Self>;

    /// Creates a new instance of the element. This should not block. Use
    /// [`Element::init`] for any long-running initialization.
    ///
    /// [`Element::init`]: crate::element::Element::init
    fn new(options: Self::Options, backend: Backend) -> Self;
}

impl<T> Options for T
where
    T: New<Options = T>,
{
    type New = T;
}
