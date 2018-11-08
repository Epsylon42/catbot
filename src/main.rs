#![feature(range_contains)]

extern crate regex;
extern crate reqwest;
extern crate serenity;
extern crate typemap;

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

mod handler;

fn main() {
    env_logger::init();

    let mut client = serenity::Client::new(
        &std::env::var("CATBOT_TOKEN").unwrap(),
        handler::CatBotHandler::new(),
    ).unwrap();
    handler::CatBotHandler::init(&mut client);

    println!("Starting");

    client.start().unwrap();
}
