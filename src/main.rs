use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;
use std::env;
use std::default::Default;
use std::fmt::{Display, Formatter, write};
use std::str;
use std::str::FromStr;
use semver::{Version, BuildMetadata, Prerelease};
use iso8601::{date, Date};

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        for attachment in msg.attachments.iter() {
            let content_type = attachment.content_type.as_ref().unwrap();
            let filename = attachment.filename.clone();

            if content_type == "text/plain; charset=utf-8" && filename.ends_with(".log") {
                let _ = msg.channel_id.say(&context, "Found a log! Parsing...").await;
                parse_log(attachment.download().await.expect("Failed to download log")).await;
            }
        }

        if msg.content == "~ping" {
            let _ = msg.channel_id.say(&context, "Pong!").await;
        }
    }

    async fn ready(&self, context: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        // let _ = ChannelId(1115961098961702992).say(&context, "Its alive!").await;
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents).event_handler(Handler).await.expect("Failed to create client");

    client.start().await.expect("Failed to start client");
}

#[derive(Default)]
enum WinVersion {
    WinXP,
    Win7,
    Win8,
    Wine81,
    #[default]
    Win10
}

impl FromStr for WinVersion {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "winxp64" => Ok(WinVersion::WinXP),
            "win7" => Ok(WinVersion::Win7),
            "win8" => Ok(WinVersion::Win8),
            "win81" => Ok(WinVersion::Wine81),
            "win10" => Ok(WinVersion::Win10),
            _ => Err(()),
        }
    }
}

#[derive(Default)]
enum EnhancedSync {
    None,
    ESync,
    #[default]
    MSync
}

impl FromStr for EnhancedSync {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "none" => Ok(EnhancedSync::None),
            "esync" => Ok(EnhancedSync::ESync),
            "msync" => Ok(EnhancedSync::MSync),
            _ => Err(()),
        }
    }
}

#[derive(Default)]
struct LogHeader<'a> {
    whisky_version: WhiskyVersion,
    date: Date,
    macos_version: WhiskyVersion,
    bottle_name: &'a str,
    bottle_url: &'a str,
    wine_version: WhiskyVersion,
    windows_version: WinVersion,
    enhanced_sync: EnhancedSync,
    metal_hud: bool,
    metal_trace: bool,
    arguments: &'a str
}

impl<'a> Display for LogHeader<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Whisky Version: {}\n", self.whisky_version.0)?;
        write!(f, "Date: {}\n", self.date)?;
        write!(f, "macOS Version: {}\n", self.macos_version.0)
    }
}

struct WhiskyVersion(Version);
impl Default for WhiskyVersion {
    fn default() -> Self {
        return WhiskyVersion {
            0: Version {
                major: 0,
                minor: 0,
                patch: 0,
                pre: Prerelease::EMPTY,
                build: BuildMetadata::EMPTY
            },
        };
    }
}

async fn parse_log(log_data: Vec<u8>) {
    let log_string = str::from_utf8(log_data.as_slice()).expect("Failed to get log as array");
    let mut log_header: LogHeader = Default::default();

    for line in  log_string.lines() {
        if line.is_empty() {
            continue;
        }

        if line.contains(":") {
            let (key, value) = line.split_once(":").unwrap();
            match key.trim() {
                "Whisky Version" => {
                    log_header.whisky_version.0 = Version::parse(value.trim()).expect("Failed to parse version");
                },
                "Date" => {
                    log_header.date = date(value.trim()).unwrap();
                },
                "macOS Version" => {
                    log_header.macos_version.0 = Version::parse(value.trim()).expect("Failed to parse version");
                },
                "Bottle Name" => {
                    log_header.bottle_name = value.trim();
                },
                "Bottle URL" => {
                    log_header.bottle_url = value.trim();
                },
                "Wine Version" => {
                    log_header.wine_version.0 = Version::parse(value.trim()).expect("Failed to parse version");
                },
                "Windows Version" => {
                    log_header.windows_version = WinVersion::from_str(value.trim()).unwrap();
                },
                "Enhanced Sync" => {
                    log_header.enhanced_sync = EnhancedSync::from_str(value.trim()).unwrap();
                },
                "Metal HUD" => {
                    log_header.metal_hud = value.trim().parse().unwrap();
                },
                "Metal Trace" => {
                    log_header.metal_trace = value.trim().parse().unwrap();
                },
                "Arguments" => {
                    log_header.arguments = value.trim();
                },
                _ => println!("Don't care")
            }
        }
    }

    println!("{}", log_header);
}