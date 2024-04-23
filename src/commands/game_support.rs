use log::info;
use reqwest::StatusCode;
use crate::{Context, Error};

#[poise::command(prefix_command, slash_command, aliases("gs"))]
pub async fn game_support(ctx: Context<'_>,
    #[description = "Game Name"] game_name: Option<String>) -> Result<(), Error> {
    // String containing default response
    let mut message = "https://docs.getwhisky.app/game-support/";

    match game_name {
        Some(name) => {
            message.push_str(&name);

            // TODO: Shuttle blocks us from making requests
            // let resp = reqwest::get(site.clone()).await?;
            //
            // info!("{} returned code {}", site, resp.status());
            //
            // if resp.status() == StatusCode::OK {
            //
            // } else {
            //     ctx.reply("Hmm, seems that game isn't in our docs.").await?;
            // }
        },
        None => {}
    }

    if let Context::Prefix(prefix) = ctx {
        match prefix.msg.clone().referenced_message {
            Some(parent) => {
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