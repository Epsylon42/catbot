use super::prelude::*;

pub struct Love;

#[async_trait]
impl Processor for Love {
    fn format(&self) -> &'static Regex {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(, )?(?i:i love you)").unwrap();
        }

        &*RE
    }

    async fn process(&self, ctx: ProcessorContext<'_>, _: Captures<'_>) -> Result<(), Error> {
        ctx.reply("I know").await?;

        Ok(())
    }
}
