use iso8601::date;
use semver::Version;
use serenity::model::prelude::*;
use serenity::prelude::*;
use shuttle_runtime;
use shuttle_secrets::SecretStore;
use std::default::Default;
use std::str;
use std::str::FromStr;

pub mod header;

struct Handler;

const ALLOWED_CHANNELS: [ChannelId; 2] = [
    ChannelId(1115957389062066218),
    ChannelId(1115961098961702992),
];

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        for attachment in msg.attachments.iter() {
            let content_type = attachment.content_type.as_ref().unwrap();
            let filename = attachment.filename.clone();

            if content_type == "text/plain; charset=utf-8" && filename.ends_with(".log") {
                let header =
                    parse_log(attachment.download().await.expect("Failed to download log")).await;
                let mut id = msg.channel_id;

                match msg.thread.clone() {
                    None => {
                        println!("Found nothing")
                    }
                    Some(channel) => id = channel.parent_id.expect("Failed to get ID"),
                }

                // if ALLOWED_CHANNELS.contains(id.as_ref()) {
                if true {
                    msg.channel_id
                        .send_message(&context, |m| {
                            m.embed(|e| {
                                e.color(0xC86432)
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
                                    })
                            })
                        })
                        .await
                        .expect("Whee");
                } else {
                    msg.channel_id
                        .say(
                            &context,
                            format!(
                                "Sorry <@{}>! I can only parse logs in <#{}> and <#{}>.",
                                msg.author.id.0, ALLOWED_CHANNELS[0].0, ALLOWED_CHANNELS[1].0
                            ),
                        )
                        .await
                        .expect("Failed to send message");
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

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let token = secret_store.get("DISCORD_TOKEN").unwrap();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Failed to create client");

    Ok(client.into())
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
                    match Version::parse(value.trim()) {
                        Err(_) => println!("Whisky Version not found!"),
                        Ok(i) => log_header.whisky_version.0 = i
                    }
                }
                "Date" => {
                    match date(value.trim()) {
                        Err(_) => println!("Date not found!"),
                        Ok(i) => log_header.date = i
                    }
                }
                "macOS Version" => {
                    match Version::parse(value.trim()) {
                        Err(_) => println!("macOS Version not found!"),
                        Ok(i) => log_header.macos_version.0 = i
                    }
                }
                "Bottle Name" => {
                    log_header.bottle_name = value.trim().to_string();
                }
                "Bottle URL" => {
                    log_header.bottle_url = value.trim().to_string();
                }
                "Wine Version" => {
                    match Version::parse(value.trim()) {
                        Err(_) => println!("Wine Version not found!"),
                        Ok(i) => log_header.wine_version.0 = i
                    }
                }
                "Windows Version" => {
                    match header::WinVersion::from_str(value.trim()) {
                        Err(_) => println!("Windows Version not found!"),
                        Ok(i) => log_header.windows_version = i
                    }
                }
                "Enhanced Sync" => {
                    match header::EnhancedSync::from_str(value.trim()) {
                        Err(_) => println!("Enhanced Sync not found!"),
                        Ok(i) => log_header.enhanced_sync = i
                    }
                }
                "Metal HUD" => {
                    match value.trim().parse() {
                        Err(_) => println!("Metal HUD not found!"),
                        Ok(i) => log_header.metal_hud = i
                    }
                }
                "Metal Trace" => {
                    match value.trim().parse() {
                        Err(_) => println!("Metal Trace not found!"),
                        Ok(i) => log_header.metal_trace = i
                    }
                }
                "Arguments" => {
                    log_header.arguments = value.trim().to_string();
                }
                _ => println!("Found unknown key '{key}'"),
            }
        }
    }

    return Box::from(log_header);
}
