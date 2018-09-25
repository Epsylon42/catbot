use super::*;
use failure::Error;
use regex::{Captures, Regex};

pub struct Colon3;

fn parse_num(text: &str) -> Result<u8, Error> {
    text
        .parse()
        .ok()
        .and_then(|num| {
            if (1..=14).contains(&num) {
                Some(num)
            } else {
                None
            }
        })
        .ok_or_else(|| {
            UserError(format_err!("Invalid pyramid size. Must be from 1 to 14")).into()
        })
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
            parse_num(cap.as_str())?
        } else {
            3
        };

        let response = if cap.get(1).is_some() {
            (1..=num)
                .chain((1..num).rev())
                .flat_map(|n| {
                    use std::iter::{once, repeat};
                    repeat(":3  ").take(n as usize).chain(once("\n"))
                }).collect::<String>()
        } else {
            (1..=num)
                .chain((1..num).rev())
                .flat_map(|n| {
                    use std::iter::{once, repeat};
                    repeat("🐱").take(n as usize).chain(once("\n"))
                }).collect::<String>()
        };

        ctx.reply(&response)?;

        Ok(())
    }
}
