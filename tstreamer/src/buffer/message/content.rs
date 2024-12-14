use crate::buffer::{
    tool::{ToolError, ToolOk},
    Buffer, Image, Result, ToolCall,
};

/// [`Content`] of a [`Message`] as one or more [`Block`]s of a specific
/// [`ContentType`].
pub trait Content: Buffer + std::fmt::Display {
    /// Iterate over the [`Block`]s of the content.
    fn blocks<'a>(&'a self) -> Box<dyn Iterator<Item = Block<'a>> + 'a>;
    /// Convert into the native kind of content.
    fn into_native(self: Box<Self>) -> NativeKind;
}
static_assertions::assert_impl_all!(dyn Content: Buffer, std::fmt::Display);
static_assertions::assert_obj_safe!(Content);

/// `NativeKind` of a [`Content`] (OpenAI, Misanthropic, etc.).
pub enum NativeKind {
    #[cfg(feature = "misanthropic")]
    MisanthropicPromptMessageContent(
        misanthropic::prompt::message::Content<'static>,
    ),
}

pub enum Block<'a> {
    /// Text block.
    Text { text: &'a str },
    /// Image block.
    Image { image: &'a dyn Image },
    /// Tool use
    ToolCall { call: &'a dyn ToolCall },
    /// Sucessful tool result
    ToolOk { ok: &'a dyn ToolOk },
    /// Error tool result
    ToolError { error: &'a dyn ToolError },
}
