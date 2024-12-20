pub mod content;
pub use content::{Block, Content};
use serde::{Deserialize, Serialize};

use crate::buffer::{tool, Buffer};

/// `Role` of a [`Message`]. On backends where all roles are not supported, the
/// role will be converted to the closest supported role. All backends do
/// support the required functionality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    /// An `Agent` role [`Message`], usually generated by the [`Inference`]
    /// [`Element`].
    ///
    /// [`Inference`]: crate::element::inference::Inference
    /// [`Element`]: crate::element::Element
    Agent,
    /// A `User` role [`Message`], usually from the end user.
    User,
    /// A `System` role [`Message`], giving the [`Agent`] instruction.
    ///
    /// [`Agent`]: Role::Agent
    System,
    /// [`Agent`] decided to use a [`Tool`].
    ///
    /// [`Tool`]: crate::element::tool::Tool,
    ToolUse,
    /// [`ToolResult`] from [`Tool`] with matching [`id`] to a [`ToolUse`].
    ///
    /// [`Tool`]: crate::element::tool::Tool,
    ToolResult,
}

impl Role {
    /// All supported [`Role`]s.
    pub const ALL: &'static [Role] = &[
        Role::Agent,
        Role::User,
        Role::System,
        Role::ToolUse,
        Role::ToolResult,
    ];
}

/// Any kind of (owned) [`Message`].
pub enum Any {
    Agent(Box<dyn AgentMessage>),
    User(Box<dyn UserMessage>),
    System(Box<dyn SystemMessage>),
    ToolUse(Box<dyn tool::Use>),
    ToolReturn(Box<dyn tool::Result>),
}

/// [`Content`] with a [`Role`].
pub trait Message: Buffer + std::fmt::Display {
    /// The [`Role`] of the message.
    fn role(&self) -> Role;
    /// The [`Content`] of the message in the form of one or more [`Block`]s.
    fn content<'a>(&'a self) -> &'a dyn Content;
    /// Discard the [`Role`] and return the [`Content`].
    fn into_content(self) -> Box<dyn Content>;
    /// Convert into the native [`Kind`] of message (of a specific backend).
    fn into_concrete(self: Box<Self>) -> Kind;
    /// Converted into a boxed [`Any`] [`Message`] to be routed to the correct
    /// handler. Classifies the message based on the role or other properties.
    ///
    /// ## Note:
    /// - In the case of Anthropic, any [`Message`] containing tool use blocks
    ///   will be considered [`ToolUse`] messages.
    /// - In the case of Anthropic, there is no system role, so system messages
    ///   handled by the [`Prompt`] element replace the `system` field with
    ///   the [`Content`] of the [`Message`].
    ///
    /// [`Prompt`]: crate::element::prompt::Prompt
    /// [`ToolUse`]: Role::ToolUse
    fn into_any(self: Box<Self>) -> Any;
}
static_assertions::assert_impl_all!(dyn Message: Buffer, std::fmt::Display);
static_assertions::assert_obj_safe!(Message);

/// [`Message`] created by a [`User`].
///
/// [`User`]: Role::User
pub trait UserMessage: Message {
    #[inline]
    fn role(&self) -> Role {
        Role::User
    }
}
static_assertions::assert_impl_all!(dyn UserMessage: Buffer, std::fmt::Display);
static_assertions::assert_obj_safe!(UserMessage);

/// Guaranted [`Agent`] role [`Message`].
pub trait AgentMessage: Message {
    #[inline]
    fn role(&self) -> Role {
        Role::Agent
    }
}

/// Guaranted [`System`] role [`Message`]. Generally this should only be set
/// once when the [`Prompt`] because of prompt caching with many backends.
pub trait SystemMessage: Message {
    fn role(&self) -> Role {
        Role::System
    }
}

// See the `crate::buffer::tool` module for the `ToolUse` and `ToolReturn` which
// are also `Message`s.

/// `Kind` of [`Message`] (for a specific backend).
pub enum Kind {
    /// A [`misanthropic::prompt::Message`]. The roles are limited in the
    /// Anthropic API, so tool calls will also unwrap to this.
    #[cfg(feature = "misanthropic")]
    MisanthropicPromptMessage(misanthropic::prompt::Message<'static>),
}
