use std::borrow::Cow;

use crate::info::{self, Info};

pub mod any;
mod html;
mod image;
mod markdown;
pub mod message;
#[cfg(feature = "misanthropic")]
pub mod misanthropic;
mod prompt;
pub mod tool;

pub use html::{Html, ToHtml};
pub use image::Image;
pub use markdown::{Markdown, ToMarkdown};
pub use message::{AgentMessage, Message, SystemMessage, UserMessage};
pub use prompt::Prompt;

/// A `Buffer` is a piece of data that can be written to a stream.
pub trait Buffer: Send + Info {
    /// Get a reference to the kinds of buffer.
    fn as_borrowed<'a>(&'a self) -> any::Borrowed<'a>;
    /// Convert the buffer into an enum of the kind of buffer.
    fn into_owned(self: Box<Self>) -> any::Owned;
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
    fn as_borrowed<'a>(&'a self) -> any::Borrowed<'a> {
        (**self).as_borrowed()
    }
    fn into_owned(self: Box<Self>) -> any::Owned {
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

impl Into<any::Owned> for Box<dyn Buffer> {
    fn into(self) -> any::Owned {
        self.into_owned()
    }
}

/// [`Error`] buffer.
pub trait Error:
    Buffer + std::fmt::Display + std::fmt::Debug + 'static
{
}
impl std::error::Error for dyn Error {}
// so ? works
impl<T> From<T> for Box<dyn Error>
where
    T: Error + 'static,
{
    fn from(err: T) -> Self {
        Box::new(err)
    }
}
static_assertions::assert_impl_all!(dyn Error: Buffer, std::error::Error);
static_assertions::assert_obj_safe!(Error);

#[derive(Debug, Clone, Copy, thiserror::Error)]
#[error("{}", inner)]
struct ErrorStaticString {
    inner: &'static str,
}

impl Error for ErrorStaticString {}

impl Buffer for ErrorStaticString {
    fn as_borrowed<'a>(&'a self) -> any::Borrowed<'a> {
        any::Borrowed::Error(self)
    }

    fn into_owned(self: Box<Self>) -> any::Owned {
        any::Owned::Error(self)
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
    fn as_borrowed<'a>(&'a self) -> any::Borrowed<'a> {
        match self {
            Ok(ok) => ok.as_borrowed(),
            Err(err) => any::Borrowed::Error(err),
        }
    }

    fn into_owned(self: Box<Self>) -> any::Owned {
        match *self {
            Ok(ok) => Box::new(ok).into_owned(),
            Err(err) => any::Owned::Error(Box::new(err)),
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

/// [`Source`]s of [`Buffer`]s.
pub mod source {
    use super::*;
    use crate::{element::prompt::PromptSource, info::Info, pad::Source};

    /// [`Message`] [`Source`]
    pub trait MessageSource: Source<Box<dyn Message>> + Info {}
    impl<T> MessageSource for T where
        T: Source<Box<dyn crate::buffer::Message>> + Info
    {
    }

    /// [`AgentMessage`] [`Source`]
    pub trait AgentMessageSource: Source<Box<dyn AgentMessage>> + Info {}
    impl<T> AgentMessageSource for T where
        T: Source<Box<dyn crate::buffer::message::AgentMessage>> + Info
    {
    }

    /// [`UserMessage`] [`Source`]
    pub trait UserMessageSource: Source<Box<dyn UserMessage>> + Info {}
    impl<T> UserMessageSource for T where
        T: Source<Box<dyn crate::buffer::message::UserMessage>> + Info
    {
    }

    /// [`tool::Schema`] [`Source`]
    pub trait ToolSchemaSource: Source<Box<dyn tool::Schema>> + Info {}
    impl<T> ToolSchemaSource for T where
        T: Source<Box<dyn crate::buffer::tool::Schema>> + Info
    {
    }

    /// [`tool::Use`] [`Source`]
    pub trait ToolUseSource: Source<Box<dyn tool::Use>> + Info {}
    impl<T> ToolUseSource for T where
        T: Source<Box<dyn crate::buffer::tool::Use>> + Info
    {
    }

    /// [`tool::Result`] [`Source`]
    pub trait ToolResultSource: Source<Box<dyn tool::Result>> + Info {}
    impl<T> ToolResultSource for T where
        T: Source<Box<dyn crate::buffer::tool::Result>> + Info
    {
    }

    /// [`Source`] capabilities of an [`Element`] (borrowed).
    ///
    /// [`Element`]: crate::element::Element
    pub enum Any<'a> {
        /// Yields [`Prompt`]s.
        Prompt(&'a dyn PromptSource),
        /// Yields [`Message`]s.
        Message(&'a dyn MessageSource),
        /// Yields [`AgentMessage`]s.
        AgentMessage(&'a dyn AgentMessageSource),
        /// Yields [`UserMessage`]s.
        UserMessage(&'a dyn UserMessageSource),
        /// Yields [`tool::Schema`]s.
        ToolSchema(&'a dyn ToolSchemaSource),
        /// Yields [`tool::Use`]s.
        ToolUse(&'a dyn ToolUseSource),
        /// Yields [`tool::Result`]s
        ToolResult(&'a dyn ToolResultSource),
    }

    impl Info for Any<'_> {
        fn name<'a>(&'a self) -> std::borrow::Cow<'a, str> {
            match self {
                Self::Prompt(e)
                | Self::Message(e)
                | Self::AgentMessage(e)
                | Self::UserMessage(e)
                | Self::ToolSchema(e)
                | Self::ToolUse(e)
                | Self::ToolResult(e) => e.name(),
            }
        }

        fn description<'a>(&'a self) -> std::borrow::Cow<'a, str> {
            match self {
                Self::Prompt(e)
                | Self::Message(e)
                | Self::AgentMessage(e)
                | Self::UserMessage(e)
                | Self::ToolSchema(e)
                | Self::ToolUse(e)
                | Self::ToolResult(e) => e.description(),
            }
        }
    }

    /// [`Source`] capabilities of an [`Element`] (mutable).
    pub enum AnyMut<'a> {
        /// Mutable [`PromptSource`].
        PromptSource(&'a mut dyn PromptSource),
        /// Mutable [`MessageSource`].
        MessageSource(&'a mut dyn MessageSource),
        /// Mutable [`AgentMessageSource`].
        AgentMessageSource(&'a mut dyn AgentMessageSource),
        /// Mutable [`UserMessageSource`].
        UserMessageSource(&'a mut dyn MessageSource),
        /// Mutable [`ToolSchemaSource`].
        ToolSchemaSource(&'a mut dyn ToolSchemaSource),
        /// Mutable [`ToolUseSource`].
        ToolUseSource(&'a mut dyn ToolUseSource),
        /// Mutable [`ToolResultSource`].
        ToolResultSource(&'a mut dyn ToolResultSource),
    }
}

pub mod sink {
    use super::*;
    use crate::pad::Sink;

    /// [`Prompt`] [`Sink`]
    pub trait PromptSink: Sink<Box<dyn Prompt>> + Info {}
    impl<T> PromptSink for T where T: Sink<Box<dyn Prompt>> + Info {}

    /// [`Message`] [`Sink`]
    pub trait MessageSink: Sink<Box<dyn Message>> + Info {}
    impl<T> MessageSink for T where T: Sink<Box<dyn Message>> + Info {}

    /// [`AgentMessage`] [`Sink`]
    pub trait AgentMessageSink: Sink<Box<dyn AgentMessage>> + Info {}
    impl<T> AgentMessageSink for T where T: Sink<Box<dyn AgentMessage>> + Info {}

    /// [`UserMessage`] [`Sink`]
    pub trait UserMessageSink: Sink<Box<dyn UserMessage>> + Info {}
    impl<T> UserMessageSink for T where T: Sink<Box<dyn UserMessage>> + Info {}

    /// [`tool::Use`] [`Sink`]
    pub trait ToolUseSink: Sink<Box<dyn tool::Use>> + Info {}

    /// All possible types of [`Sink`] elements (borrowed).
    ///
    /// [`Sink`]: crate::pad::Sink
    pub enum Any<'a> {
        /// Accepts [`Prompt`]s.
        Prompt(&'a dyn PromptSink),
        /// Accepts [`Message`]s.
        Message(&'a dyn MessageSink),
        /// Accepts [`AgentMessage`]s.
        AgentMessage(&'a dyn AgentMessageSink),
        /// Accepts [`UserMessage`]s.
        UserMessage(&'a dyn UserMessageSink),
        /// Accepts [`tool::Result`]s.
        ToolUse(&'a dyn ToolUseSink),
    }

    /// All possible types of [`Sink`] elements (mutable).
    pub enum AnyMut<'a> {
        /// A mutable [`PromptSource`] element.
        Message(&'a mut dyn MessageSink),
    }
}
