use super::*;
use failure::Error;
use regex::{Captures, Regex};

use std::collections::HashMap;

pub struct Post {
    pub map: HashMap<String, String>,
}

impl Processor for Post {
    fn format(&self) -> &'static Regex {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"post (.*)").unwrap();
        }

        &*RE
    }

    fn process(&self, ctx: ProcessorContext, cap: Captures) -> Result<(), Error> {
        if let Some(response) = self.map.get(&cap.get(1).unwrap().as_str().to_lowercase()) {
            ctx.reply(response)?;
        } else {
            ctx.reply("I'm sorry. I don't have anything like this.")?;
        }

        Ok(())
    }
}
