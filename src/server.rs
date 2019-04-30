use rocket::http::Status;
use rocket::request::Form;
use rocket::response::{content::Html, status, Redirect};
use rocket::State;
use serenity::http::{get_channel, get_channels, get_guilds, GuildPagination};
use serenity::model::{
    channel::{Channel, ChannelType},
    id::GuildId,
};

#[derive(Deserialize)]
pub struct Config {
    pass: String,
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
fn guilds() -> Result<Html<String>, serenity::Error> {
    let guilds = get_guilds(&GuildPagination::After(GuildId(0)), 100)?
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
fn guild(id: u64) -> Result<Html<String>, serenity::Error> {
    let channels = get_channels(id)?
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
fn channel(id: u64) -> Result<Html<String>, serenity::Error> {
    if let Channel::Guild(chan) = get_channel(id)? {
        let chan = chan.read();
        Ok(Html(template(
            &chan.name,
            &format!(
                r#"
<form action="/channel/{}" method="POST">
<input type="password" name="pass" placeholder="password">
<br />
<textarea name="message"></textarea>
<br />
<input type="submit" value="send">
</form>
"#,
                id
            ),
        )))
    } else {
        Err(serenity::Error::Other("Invalid channel type"))
    }
}

#[derive(FromForm)]
struct MsgForm {
    pass: String,
    message: String,
}

#[post("/channel/<id>", data = "<data>")]
fn channel_post(
    id: u64,
    data: Form<MsgForm>,
    conf: State<Config>,
) -> Result<Result<Redirect, status::Custom<Html<String>>>, serenity::Error> {
    if let Channel::Guild(chan) = get_channel(id)? {
        if data.pass == conf.pass {
            let chan = chan.read();
            chan.say(&data.message)?;

            Ok(Ok(Redirect::to(format!("/channel/{}", id))))
        } else {
            Ok(Err(status::Custom(
                Status::Forbidden,
                Html(template("Wrong password", "<h1>Wrong Password</h1>")),
            )))
        }
    } else {
        Err(serenity::Error::Other("Invalid channel type"))
    }
}

pub fn start(conf: Config) {
    println!("Starting server");
    rocket::ignite()
        .manage(conf)
        .mount("/", routes![root, guilds, guild, channel, channel_post])
        .launch();
}
