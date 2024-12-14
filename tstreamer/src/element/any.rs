use serde::{Deserialize, Serialize};

use crate::backends;

/// A `Kind` of [`Element`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Kind {
    /// A [`Prompt` `Element`]. Provides a [`Prompt`].
    ///
    /// Accepts:
    /// - [`Prompt`]s
    /// - [`Message`]s (appending to the prompt)
    /// TODO:
    /// - [`ToolSchema`]s (replacing the prompt's tool schemas)
    ///
    /// Yields:
    /// - [`Prompt`]s (copies of the prompt)
    ///
    /// [`Prompt`] [`Element`]: crate::element::prompt::Prompt
    Prompt,
    /// A [`Inference`] [`Element`], accepting [`Prompt`]s and yielding [`Agent`]
    /// [`Role`] [`Message`]s. Can also connect to a [`ToolBox`] for tool use.
    Inference,
}

/// An `Owned` [`Element`].
pub enum Owned {
    Prompt(Box<dyn crate::element::prompt::Prompt>),
    Inference(Box<dyn crate::element::inference::Inference>),
}

/// An error indicating that an [`Element`] is unavailable for a given backend.
#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
#[error(
    "UNAVAILABLE: Element `{}` is unavailable for backend `{}`.",
    kind,
    backend
)]
pub struct UnavailableError {
    /// The kind of [`Element`] that is unavailable.
    pub kind: Kind,
    /// The backend for which the [`Element`] is unavailable.
    pub backend: backends::Backend,
}

/// Configuration error when constructing an [`Element`].
#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
#[error("CONFIG: {}", message)]
pub struct ConfigError {
    /// The error message.
    pub message: String,
}

/// Error when constructing a new [`Element`].
#[derive(
    Debug, thiserror::Error, Serialize, Deserialize, derive_more::IsVariant,
)]
pub enum NewError {
    /// The [`Element`] is unavailable for the backend.
    #[error(transparent)]
    Unavailable(#[from] UnavailableError),
    /// There was a configuration error.
    #[error(transparent)]
    Config(#[from] ConfigError),
}

impl Kind {
    /// Construct a new [`Element`] of this kind for a particular backend with
    /// the given options.
    pub fn new(
        self,
        backend: backends::Backend,
        options: serde_json::Value,
    ) -> Result<Owned, NewError> {
        match self {
            Kind::Prompt => match backend {
                // TODO: A backend independent prompt
                backends::Backend::Independent => Err(UnavailableError {
                    kind: Kind::Prompt,
                    backend,
                }
                .into()),
                #[cfg(feature = "misanthropic")]
                backends::Backend::Misanthropic => {
                    let prompt: ::misanthropic::Prompt =
                        serde_json::from_value(options).map_err(|e| {
                            ConfigError {
                                message: e.to_string(),
                            }
                        })?;
                    Ok(Owned::Prompt(Box::new(prompt)))
                }
            },
        }
    }
}
