use serde::{Deserialize, Serialize};

/// A `Backend` for [`Element`]s.
// We don't use unit structs because we want to be able to use multiple backends
// at once (for example, Misanthropic for inference and OpenAI for embeddings).
// Also it's yet another trait and probably a generic parameter on all of the
// things.
#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, derive_more::IsVariant,
)]
pub enum Backend {
    /// Backend independent.
    Independent,
    /// Anthropic backend using the `misanthropic` crate.
    #[cfg(feature = "misanthropic")]
    Misanthropic,
    // TODO: OpenAI backend using the `openai-async` crate.
}

impl Backend {
    pub const ALL: &'static [Backend] = &[
        Backend::Independent,
        #[cfg(feature = "misanthropic")]
        Backend::Misanthropic,
    ];
}
