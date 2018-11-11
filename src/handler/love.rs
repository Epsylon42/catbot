use super::*;
use failure::Error;
use regex::{Captures, Regex};

pub struct Love;

impl Processor for Love {
    fn format(&self) -> &'static Regex {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(, )?(?i:i love you)").unwrap();
        }

        &*RE
    }

    fn process(&self, ctx: ProcessorContext, _: Captures) -> Result<(), Error> {
        ctx.reply("I know")?;

        Ok(())
    }
}
