use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn website(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "https://getwhisky.app/").await?;
    Ok(())
}