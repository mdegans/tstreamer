use direction::{Direction, Pulls, Pushes};

use crate::buffer::{Buffer, Error, Message};

pub mod direction {
    pub trait Direction {}
    pub struct Pushes;
    pub struct Pulls;

    impl Direction for Pushes {}
    impl Direction for Pulls {}
}

/// A [`Buffer`] `Source`. Indicates either that it is possible to [`Pull`] from
/// this source, or that it will push this type of buffer to a compatible
/// [`Sink`].
///
/// This is implemented for all [`Pull`]able sources.
#[async_trait::async_trait]
pub trait Source<Out: Buffer, D: Direction> {
    // Source does not necessarily implement Pull because it may not be possible
    // to pull from the source. For example, an event-driven source may not have
    // a pull method.
}
impl<B: Buffer> Source<B, Pulls> for dyn Pull<B> {}
static_assertions::assert_obj_safe!(Source<Box<dyn Message>, Pulls>);

/// A `Pull`able [`Source`]
#[async_trait::async_trait]
pub trait Pull<Out: Buffer> {
    /// [`Pull`]s a buffer from the source.
    async fn pull(&mut self) -> Result<Out, Box<dyn Error>>;
}
static_assertions::assert_obj_safe!(Pull<Box<dyn Message>>);

/// A [`Buffer`] `Sink`. Indicates either
#[async_trait::async_trait]
pub trait Sink<In: Buffer, D: Direction> {}
impl<B: Buffer> Sink<B, Pushes> for dyn Push<B> {}
static_assertions::assert_obj_safe!(Sink<Box<dyn Message>, Pulls>);

/// A [`Push`]able [`Sink`]
#[async_trait::async_trait]
pub trait Push<In: Buffer> {
    /// [`Push`]es a buffer to the sink.
    async fn push(&mut self, buffer: In) -> Result<(), Box<dyn Error>>;
}
static_assertions::assert_obj_safe!(Push<Box<dyn Message>>);
