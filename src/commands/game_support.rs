use log::info;
use reqwest::StatusCode;
use crate::{Context, Error};

#[poise::command(prefix_command, slash_command, aliases("gs"))]
pub async fn game_support(ctx: Context<'_>,
    #[description = "Game Name"] game_name: Option<String>) -> Result<(), Error> {
    // String containing default response
    let mut site = String::from("https://docs.getwhisky.app/game-support/");

    match game_name {
        Some(name) => {
            site.push_str(&name);

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

            ctx.reply(site).await?;
        },
        None => {
            ctx.reply(site).await?;
        }
    }

    Ok(())
}