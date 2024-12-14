use std::borrow::Cow;

use crate::buffer::any;
use crate::info::Info;

use super::{
    message::{Content, Role},
    Buffer, Error, Message,
};

/// `ToolUse` [`Block`] of a [`Content`].
pub trait Use: Buffer {
    /// ID of the tool call.
    fn id(&self) -> &str;
    /// The arguments for the tool.
    fn args(&self) -> &serde_json::Value;
}
static_assertions::assert_impl_all!(dyn Use: Buffer);
static_assertions::assert_obj_safe!(Use);

/// `Return` is a value returned by a [`ToolCall`]. It may be successful or an
/// error.
pub trait Result: Message {
    /// Role of the message.
    fn role(&self) -> Role {
        Role::ToolResult
    }

    /// ID of the tool call.
    fn id(&self) -> &str;

    // Name is not here because it is not part of the return value in all
    // supported backends. The element that calls the tool is responsible
    // for keeping track of the tool name and associating it with the
    // result.

    /// The sucessful return value of the tool.
    fn value<'a>(&'a self) -> &'a dyn Content;

    /// Whether the return is an error.
    fn is_error(&self) -> bool;
}
static_assertions::assert_impl_all!(dyn Result: Buffer);
static_assertions::assert_obj_safe!(Result);

/// `ToolOk` is a successful [`Result`].
pub trait ToolOk: Result {}
static_assertions::assert_impl_all!(dyn ToolOk: Buffer);
static_assertions::assert_obj_safe!(ToolOk);

/// `ToolError` is a result of a failed [`ToolCall`]. Intended for the
/// [`Agent`].
// This is not a buffer error because that is handled by the pipeline and this
// is intended for the Agent to handle.
pub trait ToolError: Result {
    /// The error message.
    fn message(&self) -> &str;
}

static_assertions::assert_impl_all!(dyn ToolError: Buffer);
static_assertions::assert_obj_safe!(ToolError);

impl std::fmt::Debug for Box<dyn ToolError> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(ToolError))
            .field("id", &self.id())
            .field("message", &self.message())
            .finish()
    }
}
impl std::error::Error for Box<dyn ToolError> {}
impl Error for Box<dyn ToolError> {}

/// `ToolSchema` provides the schema for a [`Tool`]. [`Schema`] is already
/// implemented for [`serde_json::Value`].
pub trait Schema: Buffer {
    /// The schema for the tool.
    fn schema(&self) -> &dyn serde::Serialize;
}
static_assertions::assert_impl_all!(dyn Schema: Buffer);
static_assertions::assert_obj_safe!(Schema);

impl Buffer for serde_json::Value {
    fn as_borrowed<'a>(&'a self) -> any::Borrowed<'a> {
        any::Borrowed::Schema(self)
    }

    fn into_owned(self: Box<Self>) -> any::Owned {
        any::Owned::Schema(self)
    }
}

impl Info for serde_json::Value {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("serde_json::Value")
    }

    fn description(&self) -> Cow<'static, str> {
        Cow::Borrowed("A JSON value.")
    }
}

impl Schema for serde_json::Value {
    fn schema(&self) -> &dyn serde::Serialize {
        self
    }
}
