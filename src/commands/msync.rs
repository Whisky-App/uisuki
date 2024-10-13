use crate::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn msync(ctx: Context<'_>) -> Result<(), Error> {
    let mut message = "## Steam download stops?
1. Kill All Bottles: Press `Command`⌘ + `Shift`⇧ + `K` or Go to `File` (near the Apple logo) -> `Kill All Bottles`
2. Change MSync to ESync: `Bottle Configuration` -> `Enhanced Sync`
3. Run Steam".to_owned();

    if let Context::Prefix(prefix) = ctx {
        match prefix.msg.clone().referenced_message {
            Some(parent) => {
                message += &format!("\n-# This command was invoked by {} using `~{}`", ctx.author().to_string().as_str(), "msync");

                parent.reply_ping(&ctx, message).await?;
                prefix.msg.delete(ctx).await?;
            },
            None => {
                message += &format!("\n-# This command was invoked using `~{}`", "msync");
                ctx.reply(message).await?;
            }
        }
    } else {
        ctx.reply(message).await?;
    }
    Ok(())
}

