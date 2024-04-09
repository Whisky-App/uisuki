# uisuki
Whisky's Discord Log Bot

## Instructions for running locally
You'll need to have a Discord application created, and a bot under that application added to a server. You can create that [here](https://discord.com/developers/applications).

After you have completed the above steps, do the following:
```
// Clone into the uisuki repository
git clone https://github.com/Whisky-App/uisuki.git

// cd to the uisuki directory
cd uisuki

// Inside the uisuki directory, create a Secrets.toml file, and add the following line,
// replacing <token> with the private token of the bot.
DISCORD_TOKEN="<token>"

// Start the bot
cargo shuttle run
```