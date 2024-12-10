use message::{Block, Content, Role};
use tool::{ToolError, ToolOk};

use super::*;

// Prompt

impl Buffer for ::misanthropic::prompt::Prompt<'static> {
    fn as_borrowed<'a>(&'a self) -> cow::Borrowed<'a> {
        cow::Borrowed::Prompt(self)
    }

    fn into_owned(self: Box<Self>) -> cow::Owned {
        cow::Owned::Prompt(self)
    }
}

impl Info for ::misanthropic::prompt::Prompt<'static> {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed(stringify!(Prompt))
    }

    fn description(&self) -> Cow<'static, str> {
        Cow::Borrowed("Prompt (misanthropic)")
    }
}

// Message

impl Buffer for ::misanthropic::prompt::Message<'static> {
    fn as_borrowed<'a>(&'a self) -> cow::Borrowed<'a> {
        cow::Borrowed::Message(self)
    }

    fn into_owned(self: Box<Self>) -> cow::Owned {
        cow::Owned::Message(self)
    }
}

impl Info for ::misanthropic::prompt::Message<'static> {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed(stringify!(Message))
    }

    fn description(&self) -> Cow<'static, str> {
        Cow::Borrowed("Message (misanthropic)")
    }
}

// Content

impl Buffer for ::misanthropic::prompt::message::Content<'static> {
    fn as_borrowed<'a>(&'a self) -> cow::Borrowed<'a> {
        cow::Borrowed::Content(self)
    }

    fn into_owned(self: Box<Self>) -> cow::Owned {
        cow::Owned::Content(self)
    }
}

impl Info for ::misanthropic::prompt::message::Content<'static> {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed(stringify!(Content))
    }

    fn description(&self) -> Cow<'static, str> {
        Cow::Borrowed("Any kind of content (misanthropic)")
    }
}

// Image

impl Buffer for ::misanthropic::prompt::message::Image<'static> {
    fn as_borrowed<'a>(&'a self) -> cow::Borrowed<'a> {
        cow::Borrowed::Image(self)
    }

    fn into_owned(self: Box<Self>) -> cow::Owned {
        cow::Owned::Image(self)
    }
}

impl Info for ::misanthropic::prompt::message::Image<'static> {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed(stringify!(Image))
    }

    fn description(&self) -> Cow<'static, str> {
        Cow::Borrowed("An image (misanthropic)")
    }
}

// Stream Event

impl Buffer for ::misanthropic::stream::Event<'static> {
    fn as_borrowed<'a>(&'a self) -> cow::Borrowed<'a> {
        todo!()
    }

    fn into_owned(self: Box<Self>) -> cow::Owned {
        todo!()
    }

    fn clone(&self) -> Box<dyn Buffer> {
        todo!()
    }
}

impl Info for ::misanthropic::stream::Event<'static> {
    fn name(&self) -> Cow<'static, str> {
        todo!()
    }

    fn description(&self) -> Cow<'static, str> {
        todo!()
    }
}

// Stream Error

impl Buffer for ::misanthropic::stream::Error {
    fn as_borrowed<'a>(&'a self) -> cow::Borrowed<'a> {
        cow::Borrowed::Error(self)
    }

    fn into_owned(self: Box<Self>) -> cow::Owned {
        cow::Owned::Error(self)
    }
}

impl Info for ::misanthropic::stream::Error {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed(stringify!(StreamError))
    }

    fn description(&self) -> Cow<'static, str> {
        Cow::Owned(format!("Stream Error (misanthropic): {}", self))
    }
}

// Client Error

impl Buffer for ::misanthropic::client::Error {
    fn as_borrowed<'a>(&'a self) -> cow::Borrowed<'a> {
        cow::Borrowed::Error(self)
    }

    fn into_owned(self: Box<Self>) -> cow::Owned {
        cow::Owned::Error(self)
    }
}

impl Info for ::misanthropic::client::Error {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed(stringify!(ClientError))
    }

    fn description(&self) -> Cow<'static, str> {
        Cow::Owned(format!("Client Error (misanthropic): {}", self))
    }
}

// Image Decode Error

impl Buffer for ::misanthropic::prompt::message::ImageDecodeError {
    fn as_borrowed<'a>(&'a self) -> cow::Borrowed<'a> {
        cow::Borrowed::Error(self)
    }

    fn into_owned(self: Box<Self>) -> cow::Owned {
        cow::Owned::Error(self)
    }
}

impl Info for ::misanthropic::prompt::message::ImageDecodeError {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed(stringify!(ImageDecodeError))
    }

    fn description(&self) -> Cow<'static, str> {
        Cow::Owned(format!("Image Decode Error (misanthropic): {}", self))
    }
}

// Tool Use

impl Buffer for ::misanthropic::tool::Use<'static> {
    fn as_borrowed<'a>(&'a self) -> cow::Borrowed<'a> {
        cow::Borrowed::ToolCall(self)
    }

    fn into_owned(self: Box<Self>) -> cow::Owned {
        cow::Owned::ToolCall(self)
    }
}

impl Info for ::misanthropic::tool::Use<'static> {
    fn name<'a>(&'a self) -> Cow<'a, str> {
        Cow::Borrowed(&self.name)
    }

    fn description(&self) -> Cow<'static, str> {
        Cow::Borrowed("Tool Use (misanthropic)")
    }
}

// Tool Call (by the Agent)

impl ToolCall for ::misanthropic::tool::Use<'static> {
    /// ID of the [`ToolCall`].
    fn id(&self) -> &str {
        &self.id
    }

    /// The arguments for the tool.
    fn args(&self) -> &serde_json::Value {
        &self.input
    }
}

// Tool Result

impl Error for ::misanthropic::tool::Result<'static> {}
impl ToolError for ::misanthropic::tool::Result<'static> {
    /// The error message.
    fn message(&self) -> &str {
        match &self.content {
            ::misanthropic::prompt::message::Content::SinglePart(cow_str) => {
                cow_str.as_ref()
            }
            ::misanthropic::prompt::message::Content::MultiPart(vec) => vec
                .first()
                .map(|block| match block {
                    ::misanthropic::prompt::message::Block::Text {
                        text: cow_str,
                        ..
                    } => Some(cow_str.as_ref()),
                    _ => None,
                })
                .flatten()
                .unwrap_or(""),
        }
    }
}

impl Buffer for ::misanthropic::tool::Result<'static> {
    fn as_borrowed<'a>(&'a self) -> cow::Borrowed<'a> {
        if self.is_error {
            cow::Borrowed::Error(self as &dyn Error)
        } else {
            cow::Borrowed::ToolOk(self as &dyn ToolOk)
        }
    }

    fn into_owned(self: Box<Self>) -> cow::Owned {
        if self.is_error {
            cow::Owned::Error(self)
        } else {
            cow::Owned::ToolOk(self)
        }
    }
}

impl Info for ::misanthropic::tool::Result<'static> {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed(stringify!(ToolResult))
    }

    fn description(&self) -> Cow<'static, str> {
        Cow::Borrowed(concat!(stringify!(ToolResult), " (misanthropic)"))
    }
}

impl tool::Return for ::misanthropic::tool::Result<'static> {
    /// ID of the tool call.
    fn id(&self) -> &str {
        &self.tool_use_id
    }

    /// The sucessful return value of the tool.
    fn value<'a>(&'a self) -> &'a dyn Content {
        &self.content
    }

    /// Whether the return is an error.
    fn is_error(&self) -> bool {
        self.is_error
    }
}

impl ToolOk for ::misanthropic::tool::Result<'static> {}

// Errors

impl Error for ::misanthropic::stream::Error {}
impl Error for ::misanthropic::client::Error {}
impl Error for ::misanthropic::prompt::message::ImageDecodeError {}

// Markdown Options

impl Into<::misanthropic::markdown::Options> for markdown::Options {
    fn into(self) -> ::misanthropic::markdown::Options {
        let markdown::Options {
            inner,
            system,
            tool_use,
            tool_results,
            attrs,
            heading_level,
        } = self;

        ::misanthropic::markdown::Options {
            inner,
            system,
            tool_use,
            tool_results,
            attrs,
            heading_level,
        }
    }
}

// Implement the crate `ToMarkdown` for the misanthropic `ToMarkdown` trait,
// which is implemented for pretty much everything in the misanthropic crate.
// This also implements `ToHtml` for the same types since there is a blanket
// implementation in our crate for that.

impl crate::buffer::markdown::ToMarkdown
    for dyn ::misanthropic::markdown::ToMarkdown
{
    fn markdown_events_custom<'a>(
        &'a self,
        options: markdown::Options,
    ) -> Box<dyn Iterator<Item = pulldown_cmark::Event<'a>> + 'a> {
        (self as &dyn ::misanthropic::markdown::ToMarkdown)
            .markdown_events_custom(options.into())
    }
}

// Prompt

impl Prompt for ::misanthropic::Prompt<'static> {
    fn set_system(
        mut self: Box<Self>,
        content: Option<Box<dyn Content>>,
    ) -> Box<dyn Prompt> {
        self.system = content.map(|content| content.into_native().into());
        self
    }

    fn append_system(
        mut self: Box<Self>,
        content: Box<dyn Content>,
    ) -> Box<dyn Prompt> {
        if let Some(existing) = &mut self.system {
            match content.into_native().into() {
                ::misanthropic::prompt::message::Content::SinglePart(
                    cow_str,
                ) => {
                    existing.push(cow_str);
                }
                ::misanthropic::prompt::message::Content::MultiPart(blocks) => {
                    // TODO add an `extend` method to Content
                    for block in blocks {
                        existing.push(block);
                    }
                }
            }
        } else {
            self.system = Some(content.into_native().into());
        }

        self
    }

    fn system<'a>(&'a self) -> Option<&'a dyn Content> {
        self.system.as_ref().map(|content| content as &dyn Content)
    }

    fn add_message(
        mut self: Box<Self>,
        message: Box<dyn Message>,
    ) -> Result<Box<dyn Prompt>, Box<dyn Error>> {
        self.messages.push(message.into_native().into());
        Ok(self)
    }

    fn extend_messages(
        mut self: Box<Self>,
        messages: Box<dyn Iterator<Item = Box<dyn Message>>>,
    ) -> Result<Box<dyn Prompt>, Box<dyn Error>> {
        self.messages
            .extend(messages.map(|message| message.into_native().into()));
        Ok(self)
    }

    fn messages<'a>(
        &'a self,
    ) -> Box<dyn ExactSizeIterator<Item = &'a dyn Message> + 'a> {
        Box::new(self.messages.iter().map(|message| message as &dyn Message))
    }
}

// Message

impl Into<::misanthropic::prompt::Message<'static>> for Box<dyn Message> {
    fn into(self) -> ::misanthropic::prompt::Message<'static> {
        match self.into_native() {
            message::NativeKind::MisanthropicPromptMessage(message) => message,
            message::NativeKind::MisanthropicResponseMessage(message) => {
                message.message
            }
        }
    }
}

impl Message for ::misanthropic::prompt::Message<'static> {
    fn role(&self) -> Role {
        pub use super::message::Role::*;
        pub use ::misanthropic::prompt::message::{self, Block};

        match self.role {
            message::Role::User => match &self.content {
                message::Content::SinglePart(_) => User,
                message::Content::MultiPart(vec) => {
                    // With Anthropic, the `User` role can contain a
                    // `ToolResult` block :/
                    if let Some(Block::ToolResult { .. }) = vec.first() {
                        ToolReturn
                    } else {
                        User
                    }
                }
            },
            message::Role::Assistant => match &self.content {
                message::Content::SinglePart(_) => Agent,
                message::Content::MultiPart(vec) => {
                    if let Some(Block::ToolUse { .. }) = vec.first() {
                        ToolUse
                    } else {
                        Agent
                    }
                }
            },
        }
    }

    fn content<'a>(&'a self) -> &'a dyn Content {
        &self.content
    }

    fn into_native(self: Box<Self>) -> crate::buffer::message::NativeKind {
        crate::buffer::message::NativeKind::MisanthropicPromptMessage(*self)
    }

    fn into_content(self) -> Box<dyn Content> {
        Box::new(self.content)
    }
}

impl Into<::misanthropic::prompt::Message<'static>> for message::NativeKind {
    fn into(self) -> ::misanthropic::prompt::Message<'static> {
        match self {
            message::NativeKind::MisanthropicPromptMessage(message) => message,
            message::NativeKind::MisanthropicResponseMessage(message) => {
                message.message
            }
        }
    }
}

// Content and Block

#[cfg(feature = "misanthropic")]
impl From<::misanthropic::prompt::message::Content<'static>>
    for message::content::NativeKind
{
    fn from(
        content: ::misanthropic::prompt::message::Content<'static>,
    ) -> Self {
        message::content::NativeKind::MisanthropicPromptMessageContent(content)
    }
}

#[cfg(feature = "misanthropic")]
impl Into<::misanthropic::prompt::message::Content<'static>>
    for message::content::NativeKind
{
    fn into(self) -> ::misanthropic::prompt::message::Content<'static> {
        match self {
            message::content::NativeKind::MisanthropicPromptMessageContent(
                content,
            ) => content,
        }
    }
}

impl Content for ::misanthropic::prompt::message::Content<'static> {
    fn blocks<'a>(&'a self) -> Box<dyn Iterator<Item = Block<'a>> + 'a> {
        match self {
            ::misanthropic::prompt::message::Content::SinglePart(cow_str) => {
                Box::new(std::iter::once(Block::Text {
                    text: cow_str.as_ref(),
                }))
            }
            ::misanthropic::prompt::message::Content::MultiPart(vec) => {
                Box::new(vec.iter().map(|block| match block {
                    ::misanthropic::prompt::message::Block::Text {
                        text: cow_str,
                        ..
                    } => Block::Text {
                        text: cow_str.as_ref(),
                    },
                    ::misanthropic::prompt::message::Block::Image {
                        image,
                        ..
                    } => Block::Image { image },
                    ::misanthropic::prompt::message::Block::ToolUse {
                        call,
                    } => Block::ToolCall { call },
                    ::misanthropic::prompt::message::Block::ToolResult {
                        result,
                    } => match result.is_error {
                        true => Block::ToolError { error: result },
                        false => Block::ToolOk { ok: result },
                    },
                }))
            }
        }
    }

    fn into_native(self: Box<Self>) -> message::content::NativeKind {
        message::content::NativeKind::MisanthropicPromptMessageContent(*self)
    }
}

// Image

impl Image for ::misanthropic::prompt::message::Image<'static> {
    fn format(&self) -> ::image::ImageFormat {
        match self {
            ::misanthropic::prompt::message::Image::Base64 {
                media_type,
                ..
            } => match media_type {
                ::misanthropic::prompt::message::MediaType::Jpeg => {
                    ::image::ImageFormat::Jpeg
                }
                ::misanthropic::prompt::message::MediaType::Png => {
                    ::image::ImageFormat::Png
                }
                ::misanthropic::prompt::message::MediaType::Gif => {
                    ::image::ImageFormat::Gif
                }
                ::misanthropic::prompt::message::MediaType::Webp => {
                    ::image::ImageFormat::WebP
                }
            },
        }
    }

    fn base64<'a>(&'a self) -> Cow<'a, str> {
        match self {
            ::misanthropic::prompt::message::Image::Base64 { data, .. } => {
                data.as_ref().into()
            }
        }
    }

    fn into_image(self) -> Result<::image::RgbaImage, Box<dyn Error>> {
        match self.decode() {
            Ok(image) => Ok(image),
            // For some reason both ? and .map_err(Box::new) don't work
            // here.
            Err(err) => Err(Box::new(err)),
        }
    }
}
