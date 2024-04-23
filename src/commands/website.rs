use crate::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn website(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("https://getwhisky.app/").await?;
    Ok(())
}