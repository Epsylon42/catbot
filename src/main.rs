#![feature(range_contains)]

extern crate regex;
extern crate reqwest;
extern crate serenity;
extern crate typemap;
extern crate unic_emoji_char as emoji;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod handler;

fn main() {
    env_logger::init();

    let config = std::env::var("CONFIG")
        .ok()
        .and_then(|path| std::fs::read_to_string(path).ok())
        .unwrap_or_else(|| "{}".to_string());

    let config: handler::Config = serde_json::de::from_str(&config).unwrap();

    let mut client = serenity::Client::new(
        &config.token.clone().or_else(|| std::env::var("CATBOT_TOKEN").ok()).expect("Set 'token' in a config file or specify it as an environment variable CATBOT_TOKEN"),
        handler::CatBotHandler::new(config),
    )
    .unwrap();
    handler::CatBotHandler::init(&mut client);

    println!("Starting");

    client.start().unwrap();
}
