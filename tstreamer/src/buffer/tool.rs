use super::{message::Content, Buffer, Error};

/// `ToolCall` [`Block`] of a [`Content`].
pub trait ToolCall: Buffer {
    /// ID of the tool call.
    fn id(&self) -> &str;
    /// The arguments for the tool.
    fn args(&self) -> &serde_json::Value;
}
static_assertions::assert_impl_all!(dyn ToolCall: Buffer);
static_assertions::assert_obj_safe!(ToolCall);

/// `Return` is a value returned by a [`ToolCall`]. It may be successful or an
/// error.
pub trait Return: Buffer {
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
static_assertions::assert_impl_all!(dyn Return: Buffer);
static_assertions::assert_obj_safe!(Return);

pub trait ToolOk: Return {}

/// `ToolError` is a result of a failed [`ToolCall`].
pub trait ToolError: Return {
    /// The error message.
    fn message(&self) -> &str;
}
static_assertions::assert_impl_all!(dyn ToolError: Buffer);

impl std::fmt::Display for Box<dyn ToolError> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message().fmt(f)
    }
}
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
