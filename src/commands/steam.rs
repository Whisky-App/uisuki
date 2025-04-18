use crate::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn steam(ctx: Context<'_>) -> Result<(), Error> {
    let mut message = "https://docs.getwhisky.app/steam".to_owned();

    if let Context::Prefix(prefix) = ctx {
        match prefix.msg.clone().referenced_message {
            Some(parent) => {
                message += &format!("\n-# This command was invoked by {} using `~{}`", ctx.author().to_string().as_str(), "steam");

                parent.reply_ping(&ctx, message).await?;
                prefix.msg.delete(ctx).await?;
            },
            None => {
                message += &format!("\n-# This command was invoked using `~{}`", "steam");
                ctx.reply(message).await?;
            }
        }
    } else {
        ctx.reply(message).await?;
    }

    Ok(())
}