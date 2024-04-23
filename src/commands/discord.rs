use crate::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn discord(ctx: Context<'_>) -> Result<(), Error> {
    let message = "https://discord.gg/CsqAfs9CnM";

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

    Ok(())
}