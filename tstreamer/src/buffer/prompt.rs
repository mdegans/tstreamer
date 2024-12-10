use message::Content;

use super::*;

/// [`Prompt`] containing messages, metadata, everything needed to prompt the
/// model. Most methods take self by value as the prompt is not usually mutated
/// in place, but passed around the pipeline. There should generally be only one
/// prompt in the pipeline at a time, although this is not enforced and like
/// most buffers, prompts can be cloned if the type implements [`Clone`].
pub trait Prompt: Buffer {
    /// Set system prompt [`Content`].
    fn set_system(
        self: Box<Self>,
        content: Option<Box<dyn Content>>,
    ) -> Box<dyn Prompt>;
    /// Append to the system prompt [`Content`].
    fn append_system(
        self: Box<Self>,
        content: Box<dyn Content>,
    ) -> Box<dyn Prompt>;
    /// Get system prompt [`Content`].
    fn system<'a>(&'a self) -> Option<&'a dyn Content>;
    /// Add a message to the prompt.
    fn add_message(
        self: Box<Self>,
        message: Box<dyn Message>,
    ) -> Result<Box<dyn Prompt>, Box<dyn Error>>;
    /// Extend the prompt with messages.
    fn extend_messages(
        self: Box<Self>,
        messages: Box<dyn Iterator<Item = Box<dyn Message>>>,
    ) -> Result<Box<dyn Prompt>, Box<dyn Error>>;
    /// Iterate over the messages in the prompt.
    fn messages<'a>(
        &'a self,
    ) -> Box<dyn ExactSizeIterator<Item = &'a dyn Message> + 'a>;
}
static_assertions::assert_impl_all!(dyn Prompt: Buffer);
static_assertions::assert_obj_safe!(Prompt);
