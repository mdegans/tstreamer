use std::borrow::Cow;

use html::ToHtml;
use markdown::ToMarkdown;
use message::Content;
use tool::ToolOk;

use super::*;

pub type CowBuffer<'a> = Cow<'a, Borrowed<'a>>;

/// An enum to hold any type of owned buffer.
pub enum Owned {
    ToMarkdown(Box<dyn ToMarkdown>),
    Markdown(Markdown),
    ToHtml(Box<dyn ToHtml>),
    Html(Html),
    Prompt(Box<dyn Prompt>),
    Message(Box<dyn Message>),
    Content(Box<dyn Content>),
    Image(Box<dyn Image>),
    ToolCall(Box<dyn ToolCall>),
    ToolOk(Box<dyn ToolOk>),
    Error(Box<dyn Error>),
}

/// An enum to hold any type of borrowed buffer.
pub enum Borrowed<'a> {
    ToMarkdown(&'a dyn ToMarkdown),
    Markdown(&'a Markdown),
    ToHtml(&'a dyn ToHtml),
    Html(&'a Html),
    Prompt(&'a dyn Prompt),
    Message(&'a dyn Message),
    Content(&'a dyn Content),
    Image(&'a dyn Image),
    ToolCall(&'a dyn ToolCall),
    ToolOk(&'a dyn ToolOk),
    Error(&'a dyn Error),
}
