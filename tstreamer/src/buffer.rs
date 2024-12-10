use std::borrow::Cow;

use crate::info::{self, Info};

pub mod cow;
mod html;
mod image;
mod markdown;
pub mod message;
#[cfg(feature = "misanthropic")]
pub mod misanthropic;
mod prompt;
mod tool;

pub use html::{Html, ToHtml};
pub use image::Image;
pub use markdown::{Markdown, ToMarkdown};
pub use message::Message;
pub use prompt::Prompt;
pub use tool::{Return, ToolCall};

/// A `Buffer` is a piece of data that can be written to a stream.
pub trait Buffer: Send + Info {
    /// Get a reference to the kind of buffer.
    fn as_borrowed<'a>(&'a self) -> cow::Borrowed<'a>;
    /// Convert the buffer into an enum of the kind of buffer.
    fn into_owned(self: Box<Self>) -> cow::Owned;
    /// Clone the buffer.
    // We can't use the `Clone` trait because it would break object safety.
    fn clone(&self) -> Box<dyn Buffer>
    where
        Self: Clone + 'static,
    {
        Box::new(Clone::clone(self))
    }
}
static_assertions::assert_impl_all!(dyn Buffer: Send);
static_assertions::assert_obj_safe!(Buffer);

impl<T: Buffer + ?Sized> Buffer for Box<T> {
    fn as_borrowed<'a>(&'a self) -> cow::Borrowed<'a> {
        (**self).as_borrowed()
    }
    fn into_owned(self: Box<Self>) -> cow::Owned {
        (*self).into_owned()
    }
}

impl<T: Buffer + ?Sized> Info for Box<T> {
    fn name(&self) -> Cow<'_, str> {
        (**self).name()
    }

    fn description(&self) -> Cow<'_, str> {
        (**self).description()
    }
}

impl Into<cow::Owned> for Box<dyn Buffer> {
    fn into(self) -> cow::Owned {
        self.into_owned()
    }
}

/// [`Error`] buffer.
pub trait Error:
    Buffer + std::fmt::Display + std::fmt::Debug + 'static
{
}
impl std::error::Error for dyn Error {}
static_assertions::assert_impl_all!(dyn Error: Buffer, std::error::Error);
static_assertions::assert_obj_safe!(Error);

#[derive(Debug, Clone, Copy, thiserror::Error)]
#[error("{}", inner)]
struct ErrorStaticString {
    inner: &'static str,
}

impl Error for ErrorStaticString {}

impl Buffer for ErrorStaticString {
    fn as_borrowed<'a>(&'a self) -> cow::Borrowed<'a> {
        cow::Borrowed::Error(self)
    }

    fn into_owned(self: Box<Self>) -> cow::Owned {
        cow::Owned::Error(self)
    }
}

impl Info for ErrorStaticString {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed(stringify!(ErrorStaticString))
    }

    fn description(&self) -> Cow<'static, str> {
        self.inner.into()
    }
}

// derive of #[from] fails to compile because of the `Error` bound.
impl From<&'static str> for ErrorStaticString {
    fn from(inner: &'static str) -> Self {
        ErrorStaticString { inner }
    }
}

/// Buffer is implemented for all [`Result`]s where the Ok and Err types are buffers.
impl<Ok, Err> Buffer for Result<Ok, Err>
where
    Ok: Buffer,
    Err: Error,
{
    fn as_borrowed<'a>(&'a self) -> cow::Borrowed<'a> {
        match self {
            Ok(ok) => ok.as_borrowed(),
            Err(err) => cow::Borrowed::Error(err),
        }
    }

    fn into_owned(self: Box<Self>) -> cow::Owned {
        match *self {
            Ok(ok) => Box::new(ok).into_owned(),
            Err(err) => cow::Owned::Error(Box::new(err)),
        }
    }
}

impl<Ok, Err> Info for Result<Ok, Err>
where
    Ok: Buffer,
    Err: Error,
{
    fn name(&self) -> Cow<'_, str> {
        match self {
            Ok(ok) => format!("Ok({})", ok.name()).into(),
            Err(err) => format!("Err({})", err.name()).into(),
        }
    }

    fn description(&self) -> Cow<'_, str> {
        match self {
            Ok(ok) => ok.description(),
            Err(err) => info::Info::description(err),
        }
    }
}
