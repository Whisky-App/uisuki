use crate::{Context, Error};

#[poise::command(prefix_command, slash_command, aliases("epic", "gog", "amazon", "prime"))]
pub async fn heroic(ctx: Context<'_>) -> Result<(), Error> {

    let default_message = "To use Whisky in the [Heroic Games Launcher](https://heroicgameslauncher.com/), follow these instructions:";

    let epic = "To play games on the Epic Games Launcher, use [Heroic](https://heroicgameslauncher.com/).";
    let gog = "To play games on the GOG store, use [Heroic](https://heroicgameslauncher.com/).";
    let amazon = "To play games from the Amazon Prime store, use [Heroic](https://heroicgameslauncher.com/).";

    let bigmessage = "After installing Heroic, navigate to the `Settings` tab in the sidebar. Select `Game Defaults` and change `Crossover/Wine Version` to `Whisky 2.3.x`.";

    let mut message = "".to_owned();
    let cmd_name = ctx.invoked_command_name();

    if cmd_name == "heroic" {
        message += default_message;
        message += "\n\n";
        message += bigmessage;
    }
    else if cmd_name == "epic" {
        message += epic;
        message += "\n\n";
        message += bigmessage;
    }
    else if cmd_name == "gog" {
        message += gog;
        message += "\n\n";
        message += bigmessage;
    }
    else if cmd_name == "amazon" || cmd_name == "prime" {
        message += amazon;
        message += "\n\n";
        message += bigmessage;
    }

    if let Context::Prefix(prefix) = ctx {
        match prefix.msg.clone().referenced_message {
            Some(parent) => {
                message += "\n\nThis command was invoked by ";
                message += ctx.author().to_string().as_str();

                parent.reply_ping(&ctx, message).await?;
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