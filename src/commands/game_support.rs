/*
    modified by hahayupgit 2024/4/9
    desc: added ability to return webpage of inputted argument
*/

use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("gs")]
async fn game_support(ctx: &Context, msg: &Message) -> CommandResult {

    // String containing default response
    let mut site = String::from("https://docs.getwhisky.app/game-support/");

    // command stripped of leading and trailing whitespace
    let mut message = msg.content.trim().to_string();

    // if command is only "~gs" or "~game_support", reply with default response
    if message == "~gs" || message == "~game_support" {
        msg.reply(&ctx, site).await?;
    }

    // if command has arguments
    else {

        // determine which command was said & remove command given
        if message.contains("~gs") {
            message = message.replace("~gs ", "");
        }
        else {
            message = message.replace("~game_support ", "");
        }

        // concatenate the argument to the original default response & reply
        site.push_str(&message);
        msg.reply(&ctx, site).await?;

        /*
            NOTE: this method does not have any checks for
            URLs that are incorrect. this should not pose
            any security threats, however may make it harder
            for people who don't know the exact URL of longer games
            (i.e. Star Wars Jedi: Fallen Order being sw-fallen-order
            or Geometry Wars 3: Dimensions Evolved being gw3-dimensions-evolved).

            if i was a better developer, i'd figure out how to handle
            this. but i'm not, so i may return to this in a future change.

            - hahayupgit 2024/4/9 
         */
    }
    Ok(())
}