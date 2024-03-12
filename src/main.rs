use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::model::prelude::*;
use serenity::prelude::*;

use shuttle_runtime;
use shuttle_secrets::SecretStore;
use log::info;

use crate::commands::ping::*;
use crate::commands::say::*;
use crate::commands::discord::*;
use crate::commands::game_support::*;
use crate::commands::github::*;
use crate::commands::website::*;

pub mod header;
mod commands;

#[group]
#[commands(ping, say, discord, game_support, github, website)]
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
                                        .footer(|f| {
                                            f.text(format!(
                                                "Log uploaded by @{}",
                                                msg.author.name
                                            ));
                                            f
                                        })
                                })
                                    .reference_message(&msg)
                            })
                            .await
                            .expect("Failed to send log analysis message");
                    }
                }
            }
        }

        if msg.content.contains("fortnite") {
            let _ = msg.reply_ping(&context, "no.").await;
        }

        if msg.content.contains("Fortnite") {
            let _ = msg.reply_ping(&context, "no.").await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

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
