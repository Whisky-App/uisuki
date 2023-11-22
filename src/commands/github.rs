use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn github(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "https://github.com/Whisky-App/Whisky").await?;
    Ok(())
}