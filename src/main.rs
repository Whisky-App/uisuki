use serenity::model::prelude::*;
use serenity::prelude::*;
use shuttle_runtime;
use shuttle_secrets::SecretStore;

pub mod header;

struct Handler;

const ALLOWED_CHANNELS: [ChannelId; 2] = [
    ChannelId(1115957389062066218),
    ChannelId(1115961098961702992),
];

const MOD_ROLE: RoleId = RoleId(1115957910284017714);

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        for attachment in msg.attachments.iter() {
            let content_type = attachment.content_type.as_ref().unwrap();
            let filename = attachment.filename.clone();

            if content_type == "text/plain; charset=utf-8" && filename.ends_with(".log") {
                let mut id = msg.channel_id;

                match msg.thread.clone() {
                    None => {
                        println!("Found nothing")
                    }
                    Some(channel) => id = channel.parent_id.expect("Failed to get ID"),
                }

                // if ALLOWED_CHANNELS.contains(id.as_ref()) {
                if true {
                    let header = header::LogHeader::parse_log(
                        attachment.download().await.expect("Failed to download log"),
                    )
                    .await;

                    match header {
                        None => {
                            msg.reply_ping(&context, "Log in invalid format!")
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
            let _ = msg.channel_id.say(&context, "Pong~").await;
        }

        if msg.content.contains("fortnite") {
            let _ = msg.reply_ping(&context, "no.").await;
        }

        if msg.content.contains("Fortnite") {
            let _ = msg.reply_ping(&context, "no.").await;
        }

        if msg.content.starts_with("~say ") {
            if msg.author.has_role(&context, msg.guild_id, MOD_ROLE) {
                let message = msg.content.replace("~say ", "");
                let _ = msg.channel_id.say(&context, message).await;
                let _ = msg.delete(&context).await;
            } else {
                let _ = msg.reply_ping(&context, "Sorry, u got no perms~").await;
                println!("User {} tried to say something", msg.author.id);
            }
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
