pub mod content;
pub use content::{Block, Content};
use serde::{Deserialize, Serialize};

use crate::buffer::Buffer;

/// `Role` of a [`Message`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    Agent,
    User,
    System,
    ToolUse,
    ToolReturn,
}

/// [`Content`] with a [`Role`]. This should be formatted with the role in mind.
pub trait Message: Buffer + std::fmt::Display {
    /// The [`Role`] of the message.
    fn role(&self) -> Role;
    /// The [`Content`] of the message.
    fn content<'a>(&'a self) -> &'a dyn Content;
    /// Discard the [`Role`] and return the [`Content`].
    fn into_content(self) -> Box<dyn Content>;
    /// Convert into the native kind of message.
    fn into_native(self: Box<Self>) -> NativeKind;
}
static_assertions::assert_impl_all!(dyn Message: Buffer, std::fmt::Display);
static_assertions::assert_obj_safe!(Message);

/// The native kind of a [`Message`].
pub enum NativeKind {
    #[cfg(feature = "misanthropic")]
    MisanthropicPromptMessage(misanthropic::prompt::Message<'static>),
    #[cfg(feature = "misanthropic")]
    MisanthropicResponseMessage(misanthropic::response::Message<'static>),
}
