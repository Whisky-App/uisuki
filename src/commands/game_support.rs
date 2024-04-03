use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("gs")]
async fn game_support(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "https://docs.getwhisky.app/game-support").await?;
    Ok(())
}