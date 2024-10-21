use crate::{Context, Error};

#[poise::command(prefix_command, slash_command, aliases("dmgw", "help"))]
pub async fn does_my_game_work(ctx: Context<'_>) -> Result<(), Error> {
    let mut message = "Does your game work? Check https://docs.getwhisky.app/game-support/ first, and search to see if anyone else has this same issue in this Discord or the GitHub repository.

Can't find your game from those sources? Check here:
<https://www.protondb.com/>
<https://appdb.winehq.org/>
<https://www.codeweavers.com/compatibility>
<https://www.applegamingwiki.com/wiki/Home>
<https://www.pcgamingwiki.com/wiki/Home>

If you've checked all of these places and still can't find an answer, feel free to create a support post in #support !

As a reminder: Most games with EasyAntiCheat and most other anti-cheats will **NOT** work without workarounds.".to_owned();
    let cmd_name = ctx.invoked_command_name();
    if let Context::Prefix(prefix) = ctx {
        match prefix.msg.clone().referenced_message {
            Some(parent) => {
                message += &format!("\n-# This command was invoked by {} using `~{}`", ctx.author().to_string().as_str(), cmd_name);

                parent.reply_ping(&ctx, message).await?;
                prefix.msg.delete(ctx).await?;
            },
            None => {
                message += &format!("\n-# This command was invoked using `~{}`", cmd_name);
                ctx.reply(message).await?;
            }
        }
    } else {
        ctx.reply(message).await?;
    }

    Ok(())
}