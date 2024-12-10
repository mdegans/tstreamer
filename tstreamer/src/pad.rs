use crate::buffer::{Buffer, Error, Message};

/// A [`Buffer`] `Source`
#[async_trait::async_trait]
pub trait Source<Out: Buffer> {
    /// Pulls a buffer from the source.
    async fn pull(&mut self) -> Result<Out, Box<dyn Error>>;
}
static_assertions::assert_obj_safe!(Source<Box<dyn Message>>);

/// A Sink [`Pad`]
pub trait Sink<In: Buffer> {
    /// Pushes a buffer to the sink.
    fn push(&mut self, buffer: In) -> Result<(), Box<dyn Error>>;
}
static_assertions::assert_obj_safe!(Sink<Box<dyn Message>>);
