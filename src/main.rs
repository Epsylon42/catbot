#![feature(range_contains)]

extern crate serenity;
extern crate regex;
extern crate reqwest;
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

    let mut client = serenity::Client::new("NDQwMTIyODA1OTMwMTY0MjI0.DcdJ0Q._z5Tbign0q9baPhbwn1Ig19hZUc", handler::CatBotHandler::new())
        .unwrap();
    handler::CatBotHandler::init(&mut client);

    println!("Starting");

    client.start().unwrap();
}
