use failure::{Error, Fail, format_err};
use regex::{Captures, Regex};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::id::{ChannelId, MessageId};
use serenity::prelude::*;

use log::{info, error};

use std::collections::{hash_map, HashMap};

use crate::Config;

mod prelude {
    pub use failure::{Error, Fail, format_err};
    pub use regex::{Captures, Regex};
    pub use lazy_static::lazy_static;
    pub use serenity::{async_trait, FutureExt, futures::TryFutureExt};
    pub use serde::Deserialize;
    pub use log::{info, error, warn};

    pub(super) use super::{Processor, ProcessorContext, UserError, ChannelMessages};
}

mod cat;
mod colon3;
mod help;
mod love;
mod no;
mod post;

#[derive(Debug, Fail)]
#[fail(display = "{}", _0)]
struct UserError(pub Error);

struct ChannelMessages(HashMap<ChannelId, Vec<MessageId>>);

impl TypeMapKey for ChannelMessages {
    type Value = Self;
}

struct ProcessorContext<'a> {
    ctx: &'a mut Context,
    msg: &'a mut Message,
}

impl<'a> ProcessorContext<'a> {
    fn new(ctx: &'a mut Context, msg: &'a mut Message) -> Self {
        ProcessorContext { ctx, msg }
    }

    async fn reply(&self, text: &str) -> Result<Message, Error> {
        match self.msg.channel_id.say(&self.ctx.http, text).await {
            Ok(msg) => {
                let mut lock = self.ctx.data.write().await;
                if let Some(messages) = lock.get_mut::<ChannelMessages>() {
                    match messages.0.entry(msg.channel_id) {
                        hash_map::Entry::Occupied(mut entry) => {
                            entry.get_mut().push(msg.id);
                        }

                        hash_map::Entry::Vacant(entry) => {
                            entry.insert(vec![msg.id]);
                        }
                    }
                }

                Ok(msg)
            },

            Err(e) => Err(format_err!("{}", e)),
        }
    }
}

#[async_trait]
trait Processor: Send + Sync {
    fn format(&self) -> &'static Regex;
    async fn process(&self, ctx: ProcessorContext<'_>, cap: Captures<'_>) -> Result<(), Error>;
}

pub struct CatBotHandler {
    processors: Vec<Box<dyn Processor>>,
}

impl CatBotHandler {
    pub fn new(config: &Config) -> Self {
        CatBotHandler {
            processors: Vec::new(),
        }
        .with_processor(Box::new(colon3::Colon3))
        .with_processor(Box::new(cat::Cat))
        .with_processor(Box::new(no::No))
        .with_processor(Box::new(help::Help))
        .with_processor(Box::new(love::Love))
        .with_processor(Box::new(post::Post {
            map: config
                .post
                .iter()
                .map(|(k, v)| (k.to_lowercase(), v.clone()))
                .collect(),
        }))
    }

    pub async fn init(client: &mut Client) {
        let mut lock = client.data.write().await;
        lock.insert::<ChannelMessages>(ChannelMessages(HashMap::new()));
    }

    fn with_processor(mut self, cmd: Box<dyn Processor>) -> Self {
        self.processors.push(cmd);
        self
    }
}

fn skip_prefix(text: &str) -> Option<&str> {
    #[cfg(debug_assertions)]
    let prefix = "cbd";

    #[cfg(not(debug_assertions))]
    let prefix = "catbot";

    if text.starts_with(prefix) {
        Some(&text[prefix.len()..].trim_start())
    } else {
        None
    }
}

impl CatBotHandler {
    async fn process_msg(&self, mut ctx: Context, mut msg: Message) -> Option<()> {
        let content = msg.content.clone();
        let text = skip_prefix(&content)?;

        let (captures, processor) = self
            .processors
            .iter()
            .find_map(|proc| Some((proc.format().captures(text)?, proc)))?;

        if let Err(e) = processor.process(ProcessorContext::new(&mut ctx, &mut msg), captures).await {
            if let Some(user_err) = e.downcast_ref::<UserError>() {
                let _ = msg.channel_id.say(&ctx.http, format!(
                    "I'm sorry {}, I'm afraid I can't do that ({})",
                    msg.author.name, user_err
                )).await;
                info!("User facing error");
                for cause in e.iter_chain() {
                    info!("Because of \"{:?}\"", cause);
                }
            } else {
                let _ = msg.channel_id.say(&ctx.http, "Internal error. What have you done?").await;
                error!("Internal error");
                for cause in e.iter_chain() {
                    error!("Because of \"{:?}\"", cause);
                }
            }
        }

        Some(())
    }
}

#[async_trait]
impl EventHandler for CatBotHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        self.process_msg(ctx, msg).await;
    }
}
