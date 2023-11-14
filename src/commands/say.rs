use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[allowed_roles("Moderator", "Admin")]
async fn say(ctx: &Context, msg: &Message) -> CommandResult {
    let message = msg.content.replace("~say ", "");
    msg.channel_id.say(&ctx, message).await?;
    msg.delete(&ctx).await?;

    Ok(())
}

