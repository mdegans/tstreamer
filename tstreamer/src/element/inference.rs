use crate::{
    buffer::{message::AgentMessage, message::UserMessage, tool, Prompt},
    element::Element,
    pad::{Sink, Source},
};

/// A [`Inference`] [`Element`] calls the actual language model with all data
/// needed to prompt the model.
///
/// Accepts:
/// - [`Prompt`]s (with all data needed to prompt the model)
/// - [`UserMessage`]s (append to the prompt)
/// - [`tool::Result`]s from [`Tool`]s matching a [`tool::Use::id`].
///
/// Yields:
/// - [`AgentMessage`]s (agent role).

pub trait Inference:
    Sink<Box<dyn UserMessage>>
    + Sink<Box<dyn tool::Schema>>
    + Sink<Box<dyn tool::Result>>
    + Sink<Box<dyn Prompt>>
    + Source<Box<dyn AgentMessage>>
    + Source<Box<dyn tool::Use>>
    + Element
{
}

pub mod misanthropic {
    use std::borrow::Cow;

    use ::misanthropic::Client;

    use crate::{
        backends::Backend, buffer, element::prompt::PromptSource, info::Info,
    };

    use super::*;

    impl Inference for Client {}

    impl Info for Client {
        fn name<'a>(&'a self) -> std::borrow::Cow<'a, str> {
            Cow::Borrowed(concat!(
                stringify!(Inference),
                " (",
                stringify!(misanthropic::Client),
                ")"
            ))
        }

        fn description<'a>(&'a self) -> std::borrow::Cow<'a, str> {
            Cow::Borrowed(concat!(
                stringify!(Inference),
                " element for the Misanthropic backend."
            ))
        }
    }

    #[async_trait::async_trait]
    impl Element for Client {
        #[inline]
        fn backend(&self) -> Backend {
            Backend::Misanthropic
        }

        fn sources<'a>(
            &'a self,
        ) -> Box<dyn Iterator<Item = buffer::source::Any<'a>> + 'a> {
            Box::new(
                [
                    buffer::source::Any::AgentMessage(self),
                    buffer::source::Any::ToolUse(self),
                ]
                .into_iter(),
            )
        }

        fn sources_mut<'a>(
            &'a mut self,
        ) -> Box<dyn Iterator<Item = buffer::source::AnyMut<'a>> + 'a> {
            Box::new(
                [
                    buffer::source::Any::AgentMessage(self),
                    buffer::source::Any::ToolUse(self),
                ]
                .into_iter(),
            )
        }

        fn sinks<'a>(
            &'a self,
        ) -> Box<dyn Iterator<Item = buffer::sink::Any<'a>> + 'a> {
            Box::new(
                [
                    buffer::sink::Any::UserMessage(self),
                    buffer::sink::Any::ToolSchema(self),
                    buffer::sink::Any::ToolResult(self),
                    buffer::sink::Any::Prompt(self),
                ]
                .into_iter(),
            )
        }

        fn sinks_mut<'a>(
            &'a mut self,
        ) -> Box<dyn Iterator<Item = buffer::sink::AnyMut<'a>> + 'a> {
            Box::new(
                [
                    buffer::sink::Any::UserMessage(self),
                    buffer::sink::Any::ToolSchema(self),
                    buffer::sink::Any::ToolResult(self),
                    buffer::sink::Any::Prompt(self),
                ]
                .into_iter(),
            )
        }
    }

    #[async_trait::async_trait]
    impl Sink<Box<dyn UserMessage>> for Client {
        async fn push(
            &mut self,
            buffer: Box<dyn UserMessage>,
        ) -> Result<(), Box<dyn Error>> {
            let native: ::misanthropic::prompt::Message<'static> =
                buffer.into_concrete().into();
            let prompt = (self as &dyn PromptSource).pull();
        }
    }
}
