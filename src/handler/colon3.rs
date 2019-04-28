use super::*;
use failure::Error;
use regex::{Captures, Regex};
use emoji;

pub struct Colon3;

fn parse_num(text: &str) -> Result<u8, Error> {
    text.parse()
        .map_err(|_| UserError(format_err!("Invalid pyramid size")).into())
}

impl Processor for Colon3 {
    fn format(&self) -> &'static Regex {
        lazy_static! {
            //Wow
            static ref RE: Regex = Regex::new(r"^(?i:(?P<kind>old |custom (?P<pat>.+) )?pyramid)(?: (?P<height>\d+))?$").unwrap();
        }

        &*RE
    }

    fn process(&self, ctx: ProcessorContext, cap: Captures) -> Result<(), Error> {
        //TODO: error reporting
        let num = if let Some(cap) = cap.name("height") {
            parse_num(cap.as_str())?
        } else {
            3
        };

        let mut pat = if let Some(kind) = cap.name("kind") {
            if kind.as_str().starts_with("old") {
                String::from(":3")
            } else {
                cap.name("pat")
                    .map(|s| s.as_str().to_string())
                    .ok_or_else(|| format_err!("Did not match a pattern"))?
            }
        } else {
            String::from("🐱")
        };

        pat.push(' ');

        let response = (1..=num)
            .chain((1..num).rev())
            .flat_map(|n| {
                use std::iter::{once, repeat};
                repeat(pat.as_str()).take(n as usize).chain(once("\n"))
            })
            .collect::<String>();

        if response.len() > 2000 {
            return Err(UserError(format_err!("Maximum message length exceeded (2000)")).into());
        }

        if response.split('\n').count() > 30 {
            return Err(UserError(format_err!("Maximum number of lines exceeded (30). It's not a discord limitation, BTW, it's you people not knowing when to stop")).into());
        }

        if response.chars().filter(|c| emoji::is_emoji(*c)).count() > 198 {
            return Err(UserError(format_err!("Maximum number of emojis exceeded (198)")).into());
        }

        ctx.reply(&response)?;

        Ok(())
    }
}
