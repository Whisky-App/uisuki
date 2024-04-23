use crate::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn discord(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("https://discord.gg/CsqAfs9CnM").await?;
    Ok(())
}