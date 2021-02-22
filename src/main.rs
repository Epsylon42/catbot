#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use serde::Deserialize;

mod handler;
mod server;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub post: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    pub server: Option<server::Config>,
}



#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();

    let config = std::env::var("CONFIG")
        .ok()
        .and_then(|path| std::fs::read_to_string(path).ok())
        .unwrap_or_else(|| "{}".to_string());

    let config: Config = serde_json::de::from_str(&config).unwrap();
    let token = config.token.clone().or_else(|| std::env::var("CATBOT_TOKEN").ok()).expect("Set 'token' in a config file or specify it as an environment variable CATBOT_TOKEN");

    let mut client = serenity::Client::builder(token)
        .event_handler(handler::CatBotHandler::new(&config))
        .await
        .unwrap();

    handler::CatBotHandler::init(&mut client).await;

    if let Some(conf) = config.server {
        tokio::spawn(server::start(conf, client.cache_and_http.http.clone()).launch());
    }

    client.start().await.unwrap();
}
