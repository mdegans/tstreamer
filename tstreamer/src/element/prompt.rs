use crate::{
    buffer::{Error, Prompt},
    pad::Source,
};

/// Misanthropic Prompt Source
#[derive(Default, derive_more::From)]
pub struct MisanthropicPromptSource {
    prompt: misanthropic::Prompt<'static>,
}

#[async_trait::async_trait]
impl Source<Box<dyn Prompt>> for MisanthropicPromptSource {
    async fn pull(&mut self) -> Result<Box<dyn Prompt>, Box<dyn Error>> {
        Ok(Box::new(self.prompt.clone()))
    }
}
