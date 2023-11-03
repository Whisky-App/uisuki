use iso8601::date;
use semver::Version;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;
use std::default::Default;
use std::env;
use std::str;
use std::str::FromStr;

pub mod header;

struct Handler;

const ALLOWED_CHANNELS: [ChannelId; 2] = [ChannelId(1115957389062066218), ChannelId(1115961098961702992)];

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        for attachment in msg.attachments.iter() {
            let content_type = attachment.content_type.as_ref().unwrap();
            let filename = attachment.filename.clone();

            if content_type == "text/plain; charset=utf-8" && filename.ends_with(".log") {
                let header = parse_log(attachment.download().await.expect("Failed to download log")).await;
                let mut id = msg.channel_id;

                msg.
                match msg.thread.clone() {
                    None => {println!("Found nothing")}
                    Some(channel) => {id = channel.parent_id.expect("Failed to get ID")}
                }

                if ALLOWED_CHANNELS.contains(id.as_ref()) {
                    msg.channel_id.send_message(&context, |m| {
                        m.embed(|e| e
                            .color(0xC86432)
                            .field("Whisky Version", header.whisky_version.0, true)
                            .field("Date", header.date.to_owned(), true)
                            .field("macOS Version", header.macos_version.0, true)
                            .field("Wine Version", header.wine_version.0, true)
                            .field("Windows Version", header.windows_version, true)
                            .field("Enhanced Sync", header.enhanced_sync, true)
                            .field("Bottle Name", header.bottle_name.clone(), false)
                            .field("Bottle URL", format!("`{}`", header.bottle_url), false)
                            .field("Arguments", format!("`{}`", header.arguments), false)
                            .footer(|f| {
                                f.text(format!("Log uploaded by @{}", msg.author.name));
                                f
                            }))
                    }).await.expect("Whee");
                } else {
                    msg.channel_id.say(&context, format!("Sorry <@{}>! I can only parse logs in <#{}> and <#{}>.", msg.author.id.0, ALLOWED_CHANNELS[0].0, ALLOWED_CHANNELS[1].0)).await.expect("Failed to send message");
                }
            }
        }

        if msg.content == "~ping" {
            let _ = msg.channel_id.say(&context, "Pong!").await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Failed to create client");

    client.start().await.expect("Failed to start client");
}

// TODO: Don't use a Box for this
async fn parse_log(log_data: Vec<u8>) -> Box<header::LogHeader> {
    let log_string = str::from_utf8(log_data.as_slice()).expect("Failed to get log as array");
    let mut log_header: header::LogHeader = Default::default();

    for line in log_string.lines() {
        if line.is_empty() {
            continue;
        }

        if line.contains(":") {
            let (key, value) = line.split_once(":").unwrap();
            match key.trim() {
                "Whisky Version" => {
                    log_header.whisky_version.0 =
                        Version::parse(value.trim()).expect("Failed to parse version");
                }
                "Date" => {
                    log_header.date = date(value.trim()).unwrap();
                }
                "macOS Version" => {
                    log_header.macos_version.0 =
                        Version::parse(value.trim()).expect("Failed to parse version");
                }
                "Bottle Name" => {
                    log_header.bottle_name = value.trim().to_string();
                }
                "Bottle URL" => {
                    log_header.bottle_url = value.trim().to_string();
                }
                "Wine Version" => {
                    log_header.wine_version.0 =
                        Version::parse(value.trim()).expect("Failed to parse version");
                }
                "Windows Version" => {
                    log_header.windows_version =
                        header::WinVersion::from_str(value.trim()).unwrap();
                }
                "Enhanced Sync" => {
                    log_header.enhanced_sync =
                        header::EnhancedSync::from_str(value.trim()).unwrap();
                }
                "Metal HUD" => {
                    log_header.metal_hud = value.trim().parse().unwrap();
                }
                "Metal Trace" => {
                    log_header.metal_trace = value.trim().parse().unwrap();
                }
                "Arguments" => {
                    log_header.arguments = value.trim().to_string();
                }
                _ => println!("Don't care"),
            }
        }
    }

    return Box::from(log_header);
}
