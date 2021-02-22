use super::prelude::*;

pub struct Help;

#[async_trait]
impl Processor for Help {
    fn format(&self) -> &'static Regex {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"(?i:help)").unwrap();
        }

        &*RE
    }

    async fn process(&self, ctx: ProcessorContext<'_>, _: Captures<'_>) -> Result<(), Error> {
        let help = "I can help you
start each command with `catbot`

Things in square brackets are optional
Things in triangle brackets are (with some limitations) whatever you want
`|` means `or`

`help`: show this message
`cat`: find a cat picture
`post <ITEM>: post something`
`[old |custom <PATTERN> ]pyramid[ <HEIGHT>]`: build a pyramid
`no`: delete latest message";

        ctx.reply(help).await?;

        Ok(())
    }
}
