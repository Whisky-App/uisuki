use poise::serenity_prelude as serenity;
type Data = ();
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

use shuttle_runtime;
use shuttle_runtime::SecretStore;
use log::info;
use serenity::all::{CreateEmbed, CreateEmbedFooter, CreateMessage};

use crate::commands::ping::*;
use crate::commands::discord::*;
use crate::commands::game_support::*;
use crate::commands::github::*;
use crate::commands::website::*;
use crate::commands::does_my_game_work::*;
use crate::commands::say::*;
use crate::commands::username::*;
use crate::commands::heroic::*;
use crate::commands::paths::*;
use crate::commands::msync::*;

pub mod header;
mod commands;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let token = secret_store.get("DISCORD_TOKEN").unwrap();
    let intents = serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .setup(|ctx, _ready, framework| Box::pin(async move {
            poise::builtins::register_globally(ctx, &framework.options().commands).await?;
            Ok(())
        }))
        .options(poise::FrameworkOptions {
            commands: vec![
                discord(),
                does_my_game_work(),
                game_support(),
                github(),
                ping(),
                website(),
                say(),
                username(),
                heroic(),
                paths(),
                msync()
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".into()),
                ..Default::default()
            },
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("Failed to create client");

    Ok(client.into())
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            info!("{} is connected!", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {
            for attachment in new_message.attachments.iter() {
                let content_type = attachment.content_type.as_ref().unwrap();
                let filename = attachment.filename.clone();

                if content_type == "text/plain; charset=utf-8" && filename.ends_with(".log") {
                    let header = header::LogHeader::parse_log(
                        attachment.download().await.expect("Failed to download log"),
                    ).await;

                    match header {
                        None => {
                            new_message.reply_ping(&ctx, "Log format is invalid!")
                                .await
                                .expect("Failed to send message");
                        }
                        Some(header) => {
                            new_message.channel_id
                                .send_message(&ctx, CreateMessage::new()
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
                                            new_message.author.name))
                                        )
                                    )
                                ).await.expect("Failed to send log analysis message");
                        }
                    }
                }
            }

            if !new_message.author.bot {
                if new_message.content.contains("fortnite") {
                    let _ = new_message.reply_ping(&ctx, "no.").await;
                }

                if new_message.content.contains("Fortnite") {
                    let _ = new_message.reply_ping(&ctx, "no.").await;
                }
            }
        }
        _ => {}
    }
    Ok(())
}
