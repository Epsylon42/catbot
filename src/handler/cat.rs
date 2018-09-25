use failure::{Error, Fail};
use regex::{Regex, Captures};
use reqwest;

use super::*;

#[derive(Deserialize)]
struct CatResponse {
    file: String
}

pub struct Cat;

impl Processor for Cat {
    fn format(&self) -> &'static Regex {
        lazy_static! {
            static ref RE: Regex = {
                let cats = [
                    "cat",
                    "feline",
                    "best animal",
                    "literally domesticated tiger",
                    "apex predator",
                ]
                    .into_iter()
                    .flat_map(|s| {
                        use std::iter::once;

                        once("|").chain(once(*s))
                    })
                    .skip(1)
                    .collect::<String>();

                Regex::new(&format!(r"^(?i:{})$", cats)).unwrap()
            };
        }

        &*RE
    }

    fn process(&self, ctx: ProcessorContext, _: Captures) -> Result<(), Error> {
        let response = reqwest::get("http://aws.random.cat/meow")
            .and_then(|mut response| {
                response.json::<CatResponse>()
                    .map(|response| response.file)
            })
            .or_else(|e| {
                error!("Main api error. Using fallback");
                reqwest::get("http://thecatapi.com/api/images/get?format=src")
                    .map(|response| response.url().as_str().to_owned())
            })
            .map_err(|e| e.context(UserError(format_err!("Could not get a cat picture"))))?;

        ctx.reply(&response)?;

        Ok(())
    }
}