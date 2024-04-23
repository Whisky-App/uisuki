#![allow(deprecated)]

use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::model::prelude::*;
use serenity::prelude::*;

use shuttle_runtime;
use shuttle_runtime::SecretStore;
use log::info;
use serenity::all::{CreateEmbed, CreateEmbedFooter, CreateMessage};
use serenity::framework::standard::Configuration;

use crate::commands::ping::*;
use crate::commands::say::*;
use crate::commands::discord::*;
use crate::commands::game_support::*;
use crate::commands::github::*;
use crate::commands::website::*;
use crate::commands::does_my_game_work::*;

pub mod header;
mod commands;

#[group]
#[commands(ping, say, discord, game_support, github, website, does_my_game_work)]
struct General;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        for attachment in msg.attachments.iter() {
            let content_type = attachment.content_type.as_ref().unwrap();
            let filename = attachment.filename.clone();

            if content_type == "text/plain; charset=utf-8" && filename.ends_with(".log") {
                let header = header::LogHeader::parse_log(
                    attachment.download().await.expect("Failed to download log"),
                ).await;

                match header {
                    None => {
                        msg.reply_ping(&context, "Log format is invalid!")
                            .await
                            .expect("Failed to send message");
                    }
                    Some(header) => {
                        msg.channel_id
                            .send_message(&context, CreateMessage::new()
                                .embed(CreateEmbed::new()
                                    .color(0xC86432)
                                    .field("Whisky Version", header.whisky_version.0.to_string(), true)
                                    .field("Date", header.date.to_string(), true)
                                    .field("macOS Version", header.macos_version.0.to_string(), true)
                                    .field("Wine Version", header.wine_version.0.to_string(), true)
                                    .field("Windows Version", header.windows_version.to_string(), true)
                                    .field("Enhanced Sync", header.enhanced_sync.to_string(), true)
                                    .field("Bottle Name", header.bottle_name.clone(), false)
                                    .field(
                                        "Bottle URL",
                                        format!("`{}`", header.bottle_url),
                                        false,
                                    )
                                    .field(
                                        "Arguments",
                                        format!("`{}`", header.arguments),
                                        false,
                                    )
                                    .footer(CreateEmbedFooter::new(format!(
                                        "Log uploaded by @{}",
                                        msg.author.name))
                                    )
                                )
                            ).await.expect("Failed to send log analysis message");
                    }
                }
            }
        }

        if !msg.author.bot {
            if msg.content.contains("fortnite") {
                let _ = msg.reply_ping(&context, "no.").await;
            }

            if msg.content.contains("Fortnite") {
                let _ = msg.reply_ping(&context, "no.").await;
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let framework = StandardFramework::new()
        .group(&GENERAL_GROUP);
    framework.configure(Configuration::new().prefix("~"));

    let token = secret_store.get("DISCORD_TOKEN").unwrap();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Failed to create client");

    Ok(client.into())
}
