use reqwest::StatusCode;
use crate::{Context, Error};

#[poise::command(prefix_command, slash_command, aliases("gs"))]
pub async fn game_support(ctx: Context<'_>,
    #[description = "Game Name"] game_name: Option<String>) -> Result<(), Error> {
    // String containing default response
    let mut message = String::from("https://docs.getwhisky.app/game-support/");
    let cmd_name = ctx.invoked_command_name();
    match game_name {
        Some(name) => {
            let esc_name = name
                .trim()
                .replace(" ", "-")
                .chars().filter(|b| b.is_alphanumeric() || *b == '-').collect::<String>()
                .to_lowercase();
            message.push_str(&esc_name);

            let resp = reqwest::get(message.clone()).await?;

            match resp.status() {
                StatusCode::OK => {
                    if let Context::Prefix(prefix) = ctx {
                        match prefix.msg.clone().referenced_message {
                            Some(parent) => {
                                parent.reply(&ctx, message).await?;
                                prefix.msg.delete(ctx).await?;
                            },
                            None => {
                                ctx.reply(message).await?;
                            }
                        }
                    } else {
                        ctx.reply(message).await?;
                    }
                },
                StatusCode::NOT_FOUND => {
                    ctx.reply("Hmm, seems that game isn't in our docs.").await?;
                },
                code => {
                    ctx.reply(format!("Hmm, seems I'm having trouble connecting to docs. ({code})"), ).await?;
                }
            }
        },
        None => {
            if let Context::Prefix(prefix) = ctx {
                match prefix.msg.clone().referenced_message {
                    Some(parent) => {
                        message += &format!("\n-# This command was invoked by {} using `~{}`", ctx.author().to_string().as_str(), cmd_name);

                        parent.reply_ping(&ctx, message).await?;
                        prefix.msg.delete(ctx).await?;
                    },
                    None => {
                        message += &format!("\n-# This command was invoked using `~{}`", cmd_name);
                        ctx.reply(message).await?;
                    }
                }
            } else {
                ctx.reply(message).await?;
            }
        }
    }

    Ok(())
}