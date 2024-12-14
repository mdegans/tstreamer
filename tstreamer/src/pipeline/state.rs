use super::Error;

pub trait Error: std::error::Error + Send + Sync + 'static {}

/// Marks a state struct as a [`Pipeline`] state where all [`Node`]s are in the
/// same state.
pub trait State {
    /// Next state for the [`Pipeline`] and all [`Node`]s.
    type Next: State;
    /// Error type for transitioning to the next state.
    type Error: Error;
}

/// Builder state for a [`Pipeline`]. All [`Node`]s are in this state.
pub struct Builder;
impl State for Builder {
    type Next = New;
    type Error = BuildError;
}

#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error("{0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("{0}")]
    Serde(#[from] serde::de::value::Error),
}
impl Error for BuildError {}

/// New state for a [`Pipeline`] after it has been built, but not necessarily
/// initialized.
pub struct New;
impl State for New {
    type Next = Ready;
    type Error = InitError;
}
#[derive(Debug, thiserror::Error)]
pub enum InitError {
    #[error("{0}")]
    // TODO: variants
    Element(String),
}
impl Error for InitError {}
impl From<Box<dyn std::error::Error>> for InitError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        InitError::Element(err.to_string())
    }
}

/// Initialized state for a [`Pipeline`]. All [`Node`]s are in this state. The
/// Pipeline is ready to be run.
pub struct Ready;
impl State for Ready {
    type Next = Shutdown;
    type Error = RunError;
}
#[derive(Debug, thiserror::Error)]
pub enum RunError {
    #[error("{0}")]
    Custom(String),
}

impl Error for RunError {}

/// Shutdown state for a [`Pipeline`]. All [`Node`]s are in this state.
pub struct Shutdown;
impl State for Shutdown {
    type Next = Builder;
    type Error = ShutdownError;
}
#[derive(Debug, thiserror::Error)]
pub enum ShutdownError {
    #[error("{0}")]
    Custom(String),
}
impl Error for ShutdownError {}
