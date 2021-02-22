use super::prelude::*;

#[derive(Deserialize)]
struct CatResponse {
    file: String,
}

pub struct Cat;

#[async_trait]
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
                .iter()
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

    async fn process(&self, ctx: ProcessorContext<'_>, _: Captures<'_>) -> Result<(), Error> {
        let response = reqwest::get("http://thecatapi.com/api/images/get?format=src")
            .map_ok(|response| response.url().to_string())
            .or_else(|_| {
                error!("Main api error. Using fallback");
                reqwest::get("http://aws.random.cat/meow").and_then(|response| {
                    response.json::<CatResponse>().map_ok(|response| response.file)
                })
            })
            .map_err(|e| e.context(UserError(format_err!("Could not get a cat picture"))))
            .await?;

        ctx.reply(&response).await?;

        Ok(())
    }
}
