use crate::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn discord(ctx: Context<'_>) -> Result<(), Error> {
    let mut message = "https://discord.gg/CsqAfs9CnM".to_owned();

    if let Context::Prefix(prefix) = ctx {
        match prefix.msg.clone().referenced_message {
            Some(parent) => {
                message += "\n\nThis command was invoked by ";
                message += ctx.author().to_string().as_str();

                parent.reply_ping(&ctx, message).await?;
                prefix.msg.delete(ctx).await?;
            },
            None => {
                ctx.reply(message).await?;
            }
        }
    } else {
        ctx.reply(message).await?;
    }

    Ok(())
}