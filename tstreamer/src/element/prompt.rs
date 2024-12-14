use crate::{
    buffer::{self, Error},
    element::Element,
    info::Info,
    new::New,
    pad::{
        direction::{Direction, Pulls},
        Pull, Push, Sink, Source,
    },
};

/// [`Prompt`] [`Source`]. Yields copies of a [`Prompt`].
///
/// [`Message`]: crate::buffer::Message
pub trait PromptSource<D: Direction>:
    Source<Box<dyn buffer::Prompt>, D> + Info
{
}
static_assertions::assert_obj_safe!(PromptSource<Pulls>);

impl<T, D: Direction> PromptSource<D> for T where
    T: Source<Box<dyn buffer::Prompt>, D> + Element
{
}

impl<D> Info for Box<dyn PromptSource<D>> {
    fn name(&self) -> std::borrow::Cow<'_, str> {
        "Prompt Source".into()
    }

    fn description(&self) -> std::borrow::Cow<'_, str> {
        "Provides copies of a prompt.".into()
    }
}

/// A [`Prompt`] [`Element`]. Yields copies of a [`Prompt`] and accepts
/// [`Message`]s which are appended to the [`Prompt`].
pub trait Prompt:
    Element
    + Push<Box<dyn buffer::Prompt>>
    + Pull<Box<dyn buffer::Prompt>>
    + Push<Box<dyn buffer::Message>>
{
}

#[cfg(feature = "misanthropic")]
mod misanthropic {
    use crate::backends::Backend;
    use crate::buffer::{sink, source, Message};

    use super::*;

    #[async_trait::async_trait]
    impl Element for ::misanthropic::Prompt<'static> {
        fn sources<'a>(
            &'a self,
        ) -> Box<dyn Iterator<Item = source::Borrowed<'a>> + 'a> {
            // This prompt source only has one source: itself.
            Box::new(std::iter::once(source::Borrowed::Prompt(self)))
        }

        fn sources_mut<'a>(
            &'a mut self,
        ) -> Box<dyn Iterator<Item = source::Mut<'a>> + 'a> {
            Box::new(std::iter::once(source::Mut::PromptSource(self)))
        }

        fn sinks<'a>(
            &'a self,
        ) -> Box<dyn Iterator<Item = sink::Borrowed<'a>> + 'a> {
            Box::new(std::iter::once(sink::Borrowed::MessageSink(self)))
        }

        fn sinks_mut<'a>(
            &'a mut self,
        ) -> Box<dyn Iterator<Item = sink::Mut<'a>> + 'a> {
            Box::new(std::iter::once(sink::Mut::MessageSink(self)))
        }

        fn backend(&self) -> Backend {
            Backend::Misanthropic
        }
    }

    // It's possible to pull a prompt from the prompt source.
    #[async_trait::async_trait]
    impl Pull<Box<dyn buffer::Prompt>> for ::misanthropic::Prompt<'static> {
        /// Pulls a [`Box<dyn Prompt>`] copy of the [`Prompt`].
        ///
        /// # Errors
        /// - Cannot fail, however the [`Prompt`] may be empty.
        ///
        /// [`Prompt`]: crate::buffer::Prompt
        async fn pull(
            &mut self,
        ) -> Result<Box<dyn buffer::Prompt>, Box<dyn Error>> {
            Ok(Box::new(self.clone()))
        }
    }

    // It's possible to push a message to the prompt.
    #[async_trait::async_trait]
    impl Push<Box<dyn Message>> for ::misanthropic::Prompt<'static> {
        /// Append a [`Box<dyn Message>`] to the [`Prompt`].
        ///
        /// [`Prompt`]: crate::buffer::Prompt
        ///
        /// # Errors
        /// - If the message cannot be appended to the [`Prompt`] (for example,
        ///   if the turn order is incorrect).
        async fn push(
            &mut self,
            message: Box<dyn Message>,
        ) -> Result<(), Box<dyn Error>> {
            Ok(self.push_message(message.into_concrete())?)
        }
    }

    // It's possible to replace the prompt with a new prompt.
    #[async_trait::async_trait]
    impl Push<Box<dyn buffer::Prompt>> for ::misanthropic::Prompt<'static> {
        /// Replace the [`Prompt`] with a new [`Box<dyn Prompt>`].
        ///
        /// [`Prompt`]: crate::buffer::Prompt
        ///
        /// # Errors
        /// - If the new [`Prompt`] cannot be set (for example, if the turn
        ///   order is incorrect).
        async fn push(
            &mut self,
            prompt: Box<dyn buffer::Prompt>,
        ) -> Result<(), Box<dyn Error>> {
            Ok(self.replace(prompt.into_concrete())?)
        }
    }

    impl Prompt for ::misanthropic::Prompt<'static> {}

    static_assertions::assert_impl_all!(::misanthropic::Prompt<'static>: PromptSource<Pulls>);

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn test_prompt_push_pull() {
            let message: ::misanthropic::prompt::Message =
                (::misanthropic::prompt::message::Role::User, "Test Message")
                    .into();

            let mut source = Box::new(::misanthropic::Prompt::default());

            source.push(Box::new(message)).await.unwrap();
            let prompt = source.pull().await.unwrap();
            let message = prompt.messages().next().unwrap();
            assert!(matches!(
                message.role(),
                crate::buffer::message::Role::User
            ));
            assert_eq!(format!("{}", message.content()), "Test Message");
        }
    }
}
