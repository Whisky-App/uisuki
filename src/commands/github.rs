use crate::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn github(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("https://github.com/Whisky-App/Whisky").await?;
    Ok(())
}