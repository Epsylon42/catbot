use super::prelude::*;

pub struct OwO;

#[async_trait]
impl Processor for OwO {
    fn format(&self) -> &'static Regex {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?i:owo)").unwrap();
        }

        &*RE
    }

    async fn process(&self, ctx: ProcessorContext<'_>, _: Captures<'_>) -> Result<(), Error> {
        let owo = "
🌑🌑🌑🌑🌑🌑🌑🌑🌑🌑🌑🌑🌑🌑🌑🌑🌑
🌑🌓🌕🌕🌗🌑🌔🌑🌑🌑🌖🌑🌓🌕🌕🌗🌑
🌑🌓🌘🌒🌗🌑🌓🌘🌕🌒🌗🌑🌓🌘🌒🌗🌑
🌑🌓🌘🌒🌗🌑🌑🌕🌕🌕🌑🌑🌓🌘🌒🌗🌑
🌑🌓🌕🌕🌗🌑🌑🌓🌑🌗🌑🌑🌓🌕🌕🌗🌑
🌑🌑🌑🌑🌑🌑🌑🌑🌑🌑🌑🌑🌑🌑🌑🌑🌑".trim();

        ctx.reply(owo).await?;
        Ok(())
    }
}
