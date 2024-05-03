use crate::{Context, Error};

#[poise::command(prefix_command, slash_command, aliases("dmgw"))]
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

    if let Context::Prefix(prefix) = ctx {
        match prefix.msg.clone().referenced_message {
            Some(parent) => {
                message += "\n\nThis command was invoked by ";
                message += ctx.author().to_string().as_str();

                parent.reply(&ctx, message).await?;
                prefix.msg.delete(ctx).await?;
            },
            None => {
                ctx.reply(message).await?;
            }
        }
    } else {
        ctx.reply(message).await?;
    }

    Ok(())
}