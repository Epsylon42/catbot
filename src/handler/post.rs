use super::prelude::*;

use std::collections::HashMap;

pub struct Post {
    pub map: HashMap<String, String>,
}

#[async_trait]
impl Processor for Post {
    fn format(&self) -> &'static Regex {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"post (.*)").unwrap();
        }

        &*RE
    }

    async fn process(&self, ctx: ProcessorContext<'_>, cap: Captures<'_>) -> Result<(), Error> {
        if let Some(response) = self.map.get(&cap.get(1).unwrap().as_str().to_lowercase()) {
            ctx.reply(response).await?;
        } else {
            ctx.reply("I'm sorry. I don't have anything like this.").await?;
        }

        Ok(())
    }
}
