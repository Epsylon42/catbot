use super::prelude::*;

pub struct No;

#[async_trait]
impl Processor for No {
    fn format(&self) -> &'static Regex {
        lazy_static! {
            static ref RE: Regex =
                Regex::new("(?i:^(please |pls |plz |i beg you)?(no|stop|undo)!*)$").unwrap();
        }

        &*RE
    }

    async fn process(&self, ctx: ProcessorContext<'_>, _: Captures<'_>) -> Result<(), Error> {
        let mut lock = ctx.ctx.data.write().await;
        if let Some(id) = lock
            .get_mut::<ChannelMessages>()
            .and_then(|messages| messages.0.get_mut(&ctx.msg.channel_id))
            .and_then(Vec::pop)
        {
            ctx.msg
                .channel_id
                .delete_message(&ctx.ctx.http, id)
                .await
                .map_err(|e| format_err!("{}", e))?;
        }

        Ok(())
    }
}
