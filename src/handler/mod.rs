use failure::Error;
use regex::{Regex, Captures};
use serenity::prelude::*;
use serenity::model::id::{MessageId, ChannelId};
use serenity::model::channel::Message;

use std::collections::HashMap;

mod colon3;
mod cat;
mod no;

#[derive(Debug, Fail)]
#[fail(display = "Error: {}", _0)]
struct UserError(pub Error);

struct LastChannelMessage(HashMap<ChannelId, MessageId>);

impl ::typemap::Key for LastChannelMessage {
    type Value = Self;
}

struct ProcessorContext<'a> {
    ctx: &'a mut Context,
    msg: &'a mut Message,
}

impl <'a> ProcessorContext<'a> {
    fn new(ctx: &'a mut Context, msg: &'a mut Message) -> Self {
        ProcessorContext {
            ctx,
            msg,
        }
    }

    fn reply(&self, text: &str) -> Result<Message, Error> {
        self.msg.channel_id.say(text)
            .map(|msg| {
                let mut lock = self.ctx.data.lock();
                if let Some(lcm) = lock.get_mut::<LastChannelMessage>() {
                    lcm.0.insert(msg.channel_id, msg.id);
                }

                msg
            })
            .map_err(|e| format_err!("{}", e))
    }
}

trait Processor: Send + Sync {
    fn format(&self) -> &'static Regex;
    fn process(&self, ProcessorContext, cap: Captures) -> Result<(), Error>;
}

pub struct CatBotHandler {
    processors: Vec<Box<Processor>>,
}

impl CatBotHandler {
    pub fn new() -> Self {
        CatBotHandler {
            processors: Vec::new()
        }.with_processor(Box::new(colon3::Colon3))
         .with_processor(Box::new(cat::Cat))
         .with_processor(Box::new(no::No))
    }

    pub fn init(client: &mut Client) {
        let mut lock = client.data.lock();
        lock.insert::<LastChannelMessage>(LastChannelMessage(HashMap::new()));
    }

    fn with_processor(mut self, cmd: Box<Processor>) -> Self {
        self.processors.push(cmd);
        self
    }
}

fn skip_whitespace(text: &str) -> &str {
    for (index, ch) in text.char_indices() {
        if !ch.is_whitespace() {
            return &text[index..]
        }
    }

    ""
}

fn skip_prefix(text: &str) -> Option<&str> {
    if text.starts_with("catbot") {
        Some(skip_whitespace(&text[6..]))
    } else {
        None
    }
}

impl EventHandler for CatBotHandler {
    fn message(&self, mut ctx: Context, mut msg: Message) {
        let content = msg.content.clone();
        if let Some(text) = skip_prefix(&content) {
            for processor in &self.processors {
                if let Some(captures) = processor.format().captures(text) {
                    if let Err(e) = processor.process(ProcessorContext::new(&mut ctx, &mut msg), captures) {
                        if let Some(user_err) = e.downcast_ref::<UserError>() {
                            let _ = msg.channel_id.say(format!("{}", user_err));
                            error!("User facing error");
                        } else {
                            error!("Internal error");
                        }
                        for cause in e.causes() {
                            error!("Because of \"{:?}\"", cause);
                        }
                    }
                }
            }
        }
    }
}
