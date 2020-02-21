# catbot

This Discord bot can post random cat pictures and build emoji pyramids

It can also be configured to start an http server one can use to write messages on the bot's behalf

## usage

Specify a config file in the environment variable `CONFIG`. The file is in the json format and looks like this:
```json
{
    "token": "*discord bot token goes here*",
    "server": {
        "address": [127, 0, 0, 1],
        "port": 80
    }
}
```

`server` is optional
