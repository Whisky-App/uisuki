use crate::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn username(ctx: Context<'_>) -> Result<(), Error> {
    let mut message = "Your username contains a `.` which is a special character. This can cause issues with Wine.

Follow this Apple support guide to change your username: https://support.apple.com/en-us/102547".to_owned();

    if let Context::Prefix(prefix) = ctx {
        match prefix.msg.clone().referenced_message {
            Some(parent) => {
                message += &format!("\n-# This command was invoked by {} using `~{}`", ctx.author().to_string().as_str(), "username");

                parent.reply_ping(&ctx, message).await?;
                prefix.msg.delete(ctx).await?;
            },
            None => {
                message += &format!("\n-# This command was invoked using `~{}`", "username");
                ctx.reply(message).await?;
            }
        }
    } else {
        ctx.reply(message).await?;
    }

    Ok(())
}
