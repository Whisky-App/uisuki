/*
    created by hahayupgit 2024/4/16
    desc: command for if someone asks "does x game work?"
*/
use serenity::all::EditMessage;
use serenity::futures::StreamExt;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::Duration;

#[command]
#[aliases("dmgw")]
async fn does_my_game_work(ctx: &Context, msg: &Message) -> CommandResult {

    let reply = String::from("Does your game work? Check https://docs.getwhisky.app/game-support/ first, and search to see if anyone else has this same issue in this Discord or the GitHub repository.

    Can't find your game from those sources? Check here:
    https://www.protondb.com/
    https://appdb.winehq.org/
    https://www.codeweavers.com/compatibility
    https://www.applegamingwiki.com/wiki/Home
    https://www.pcgamingwiki.com/wiki/Home
    
    If you've checked all of these places and still can't find an answer, feel free to create a support post in #support !

    As a reminder: Most games with EasyAntiCheat and most other anti-cheats will **NOT** work without workarounds.");

    let mut reply_msg = msg.reply(&ctx, reply).await?;

    // below code pulled directly from serenity-rs documentation
    // can be found here: https://docs.rs/serenity/latest/serenity/builder/struct.EditMessage.html#method.suppress_embeds

    // When the embed appears, a MessageUpdate event is sent and we suppress 
    // the embed. No MessageUpdate event is sent if the message contains no 
    // embeddable link or if the link has been posted before and is still 
    // cached in Discord's servers (in which case the embed appears 
    // immediately), no MessageUpdate event is sent. To not wait forever in
    // those cases, a timeout of 2000ms was added.
    let msg_id = reply_msg.id;
    let mut message_updates = serenity::collector::collect(&ctx.shard, move |ev| match ev {
        Event::MessageUpdate(x) if x.id == msg_id => Some(()),
        _ => None,
    });
    let _ = tokio::time::timeout(Duration::from_millis(2000), message_updates.next()).await;
    reply_msg.edit(&ctx, EditMessage::new().suppress_embeds(true)).await?;
    Ok(())
}