use failure::Error;
use regex::{Regex, Captures};
use super::*;

pub struct Colon3;

fn parse_num(text: &str) -> Result<u8, Error> {
    let num = text.parse()?;
    ensure!((1..=14).contains(num), "Invalid pyramid size {}. Must be from 1 to 14", num);
    Ok(num)
}

impl Processor for Colon3 {
    fn format(&self) -> &'static Regex {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?i:(old )?pyramid)(?: (\d+))?$").unwrap();
        }

        &*RE
    }

    fn process(&self, ctx: ProcessorContext, cap: Captures) -> Result<(), Error> {
        //TODO: error reporting
        let num = if let Some(cap) = cap.get(2) {
            parse_num(cap.as_str()).map_err(UserError)?
        } else {
            3
        };

        let response = if cap.get(1).is_some() {
            (1..=num).chain((1..num).rev())
                .flat_map(|n| {
                    use std::iter::{repeat, once};
                    repeat(":3  ").take(n as usize).chain(once("\n"))
                })
                .collect::<String>()
        } else {
            (1..=num).chain((1..num).rev())
                .flat_map(|n| {
                    use std::iter::{repeat, once};
                    repeat("🐱").take(n as usize).chain(once("\n"))
                })
                .collect::<String>()
        };

        ctx.reply(&response)?;

        Ok(())
    }
}
