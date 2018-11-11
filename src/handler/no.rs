use super::*;
use failure::Error;
use regex::{Captures, Regex};

pub struct No;

impl Processor for No {
    fn format(&self) -> &'static Regex {
        lazy_static! {
            static ref RE: Regex =
                Regex::new("(?i:^(please |pls |plz |i beg you)?(no|stop|undo)!*)$").unwrap();
        }

        &*RE
    }

    fn process(&self, ctx: ProcessorContext, _: Captures) -> Result<(), Error> {
        let mut lock = ctx.ctx.data.lock();
        if let Some(id) = lock
            .get_mut::<ChannelMessages>()
            .and_then(|messages| messages.0.get_mut(&ctx.msg.channel_id))
            .and_then(Vec::pop)
        {
            ctx.msg
                .channel_id
                .delete_message(id)
                .map_err(|e| format_err!("{}", e))?;
        }

        Ok(())
    }
}
