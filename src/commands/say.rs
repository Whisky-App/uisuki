use crate::{Context, Error};

#[poise::command(prefix_command, owners_only, guild_only)]
pub async fn say(ctx: Context<'_>,
    #[description = "Message"] message: String) -> Result<(), Error> {

    if let Context::Prefix(prefix) = ctx {
        match prefix.msg.clone().referenced_message {
            Some(parent) => {
                parent.reply(&ctx, message).await?;
            },
            None => {
                ctx.say(message).await?;
            }
        }

        prefix.msg.delete(ctx).await?;
    }

    Ok(())
}

