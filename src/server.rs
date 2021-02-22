use rocket::request::{Form, FromForm};
use rocket::response::{status, content::Html, Redirect};
use rocket::http::Status;
use serenity::http::{Http, GuildPagination};
use serenity::model::{
    channel::{Channel, ChannelType},
    id::GuildId,
};
use serde::Deserialize;

use std::sync::Arc;

#[derive(Deserialize)]
pub struct Config {
    address: [u8; 4],
    port: u16,
}

#[derive(Debug)]
struct ServerError(serenity::Error);

impl<'a> rocket::response::Responder<'a, 'static> for ServerError {
    fn respond_to(self, request: &'a rocket::request::Request<'_>) -> rocket::response::Result<'static> {
        status::Custom(Status::InternalServerError, format!("{:?}", self.0)).respond_to(request)
    }
}

impl From<serenity::Error> for ServerError {
    fn from(e: serenity::Error) -> Self {
        ServerError(e)
    }
}

fn template(title: &str, body: &str) -> String {
    format!(
        r#"
<!doctype html>
<html>
<head>
<title>{}</title>
</head>
<body>
{}
</body>
"#,
        title, body
    )
}

#[get("/")]
fn root() -> Redirect {
    Redirect::to("/guilds")
}

#[get("/guilds")]
async fn guilds(http: rocket::State<'_, Arc<Http>>) -> Result<Html<String>, ServerError> {
    let guilds = http.get_guilds(&GuildPagination::After(GuildId(0)), 100).await?
        .into_iter()
        .map(|guild| {
            format!(
                r#"<li><a href="/guilds/{}">{}</a></li>"#,
                guild.id.0, guild.name
            )
        })
        .collect::<String>();

    Ok(Html(template(
        "guilds",
        &format!("<h1>Guilds</h1><ul>{}</ul>", guilds),
    )))
}

#[get("/guilds/<id>")]
async fn guild(id: u64, http: rocket::State<'_, Arc<Http>>) -> Result<Html<String>, ServerError> {
    let channels = http.get_channels(id).await?
        .into_iter()
        .filter(|chan| chan.kind == ChannelType::Text)
        .map(|chan| {
            format!(
                r#"<li><a href="/channel/{}">{}</a></li>"#,
                chan.id.0, chan.name
            )
        })
        .collect::<String>();

    Ok(Html(template(
        "channels",
        &format!("<h1>Channels</h1><ul>{}</ul>", channels),
    )))
}

#[get("/channel/<id>")]
async fn channel(id: u64, http: rocket::State<'_, Arc<Http>>) -> Result<Html<String>, ServerError> {
    if let Channel::Guild(chan) = http.get_channel(id).await? {
        Ok(Html(template(
            &chan.name,
            &format!(
                r#"
<form action="/channel/{}" method="POST">
<textarea name="message"></textarea>
<br />
<input type="submit" value="send">
</form>
"#,
                id
            ),
        )))
    } else {
        Err(ServerError(serenity::Error::Other("Invalid channel type")))
    }
}

#[derive(FromForm)]
struct MsgForm {
    message: String,
}

#[post("/channel/<id>", data = "<data>")]
async fn channel_post(
    id: u64,
    data: Form<MsgForm>,
    http: rocket::State<'_, Arc<Http>>
) -> Result<Redirect, ServerError> {
    if let Channel::Guild(chan) = http.get_channel(id).await? {
        chan.say(&*http, &data.message).await?;

        Ok(Redirect::to(format!("/channel/{}", id)))
    } else {
        Err(ServerError(serenity::Error::Other("Invalid channel type")))
    }
}

pub fn start(conf: Config, http: Arc<Http>) -> rocket::Rocket {
    let mut server_conf = rocket::Config::release_default();
    server_conf.address = conf.address.into();
    server_conf.port = conf.port;

    println!("Starting server");
    rocket::Rocket::custom(server_conf)
        .manage(http)
        .mount("/", routes![root, guilds, guild, channel, channel_post])
}
