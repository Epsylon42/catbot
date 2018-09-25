use super::*;
use failure::Error;
use regex::{Captures, Regex};

pub struct No;

impl Processor for No {
    fn format(&self) -> &'static Regex {
        lazy_static! {
            static ref RE: Regex =
                Regex::new("(?i:(please |pls |plz |i beg you)?(no|stop|undo)!*)").unwrap();
        }

        &*RE
    }

    fn process(&self, ctx: ProcessorContext, _: Captures) -> Result<(), Error> {
        let lock = ctx.ctx.data.lock();
        if let Some(lcm) = lock.get::<LastChannelMessage>() {
            if let Some(last_msg_id) = lcm.0.get(&ctx.msg.channel_id) {
                ctx.msg
                    .channel_id
                    .delete_message(*last_msg_id)
                    .map_err(|e| format_err!("{}", e))?;
            }
        }

        Ok(())
    }
}
