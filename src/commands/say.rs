use crate::{Context, Error};

#[poise::command(prefix_command, slash_command, owners_only, guild_only)]
pub async fn say(ctx: Context<'_>,
    #[description = "Message"] message: String) -> Result<(), Error> {
    ctx.say(message).await?;

    Ok(())
}

