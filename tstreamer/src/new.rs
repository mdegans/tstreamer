use serde::{de::DeserializeOwned, Serialize};

use crate::info::Info;

/// A trait for options of an [`Element`].
pub trait Options: Serialize + DeserializeOwned + Sized + 'static {
    /// The element constructor type.
    type ElementNew: New;
}

/// A trait for constructing elements in a [`Pipeline`].
///
/// [`Pipeline`]: crate::pipeline::Pipeline
pub trait New {
    /// The type of the options.
    type Options: Options;
    /// The type of the element.
    type Element: Info;

    /// Creates a new instance of the element. This should not block. Use
    /// [`Element::init`] for any long-running initialization.
    fn new(options: Self::Options) -> Self::Element;
}
