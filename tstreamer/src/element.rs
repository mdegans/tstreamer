/// Emumerations of the different types of elements in a pipeline.
pub mod any;
/// [`Inference`] [`Element`]s.
pub mod inference;
/// [`Prompt`] containing all messages and metadata needed to prompt the model.
pub mod prompt;

use crate::backends::Backend;
use crate::buffer;
use crate::info::Info;

/// A trait for elements in a [`Pipeline`].
#[async_trait::async_trait]
pub trait Element: Info + Send + 'static {
    /// Initialize the `Element`. Get it ready to accept or produce data.
    async fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// Stop the `Element`. Stop accepting buffers. Clean up resources that
    /// should not run in a `drop`.
    async fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// Backend used by the `Element`.
    fn backend(&self) -> Backend;

    /// Iterate through the `Element`'s [`Source`]s.
    fn sources<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = buffer::source::Any<'a>> + 'a>;

    /// Iterate through the `Element`'s [`Source`]s mutably.
    fn sources_mut<'a>(
        &'a mut self,
    ) -> Box<dyn Iterator<Item = buffer::source::AnyMut<'a>> + 'a>;

    /// Iterate through the `Element`'s [`Sink`]s.
    fn sinks<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = buffer::sink::Any<'a>> + 'a>;

    /// Iterate through the `Element`'s [`Sink`]s mutably.
    fn sinks_mut<'a>(
        &'a mut self,
    ) -> Box<dyn Iterator<Item = buffer::sink::AnyMut<'a>> + 'a>;
}
static_assertions::assert_obj_safe!(Element);
