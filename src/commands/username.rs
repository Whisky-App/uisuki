use crate::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn username(ctx: Context<'_>) -> Result<(), Error> {
    let message = "Your username contains a `.` which is a special character. This can cause issues with Wine.

Follow this Apple support guide to change your username: https://support.apple.com/en-us/102547";

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
