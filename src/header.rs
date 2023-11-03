pub use crate::header::types::{EnhancedSync, WhiskyVersion, WinVersion};
use iso8601::{date, Date};
use semver::Version;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub mod types;

#[derive(Default, PartialEq)]
pub struct LogHeader {
    pub whisky_version: WhiskyVersion,
    pub date: Date,
    pub macos_version: WhiskyVersion,
    pub bottle_name: String,
    pub bottle_url: String,
    pub wine_version: WhiskyVersion,
    pub windows_version: WinVersion,
    pub enhanced_sync: EnhancedSync,
    pub metal_hud: bool,
    pub metal_trace: bool,
    pub arguments: String,
}

impl LogHeader {
    pub async fn parse_log(log_data: Vec<u8>) -> Option<Self> {
        let log_string = String::from_utf8(log_data).expect("Failed to get log as array");
        let mut log_header: LogHeader = Default::default();

        for line in log_string.lines() {
            if line.is_empty() {
                continue;
            }

            if line.contains(":") {
                let (key, value) = line.split_once(":").unwrap();
                match key.trim() {
                    "Whisky Version" => match Version::parse(value.trim()) {
                        Err(_) => println!("Whisky Version not found!"),
                        Ok(i) => log_header.whisky_version.0 = i,
                    },
                    "Date" => match date(value.trim()) {
                        Err(_) => println!("Date not found!"),
                        Ok(i) => log_header.date = i,
                    },
                    "macOS Version" => match Version::parse(value.trim()) {
                        Err(_) => println!("macOS Version not found!"),
                        Ok(i) => log_header.macos_version.0 = i,
                    },
                    "Bottle Name" => {
                        log_header.bottle_name = value.trim().to_owned();
                    }
                    "Bottle URL" => {
                        log_header.bottle_url = value.trim().to_owned();
                    }
                    "Wine Version" => match Version::parse(value.trim()) {
                        Err(_) => println!("Wine Version not found!"),
                        Ok(i) => log_header.wine_version.0 = i,
                    },
                    "Windows Version" => match WinVersion::from_str(value.trim()) {
                        Err(_) => println!("Windows Version not found!"),
                        Ok(i) => log_header.windows_version = i,
                    },
                    "Enhanced Sync" => match EnhancedSync::from_str(value.trim()) {
                        Err(_) => println!("Enhanced Sync not found!"),
                        Ok(i) => log_header.enhanced_sync = i,
                    },
                    "Metal HUD" => match value.trim().parse() {
                        Err(_) => println!("Metal HUD not found!"),
                        Ok(i) => log_header.metal_hud = i,
                    },
                    "Metal Trace" => match value.trim().parse() {
                        Err(_) => println!("Metal Trace not found!"),
                        Ok(i) => log_header.metal_trace = i,
                    },
                    "Arguments" => {
                        log_header.arguments = value.trim().to_owned();
                    }
                    _ => println!("Found unknown key '{key}'"),
                }
            }
        }

        // Log was not parsed at all, probably not right format
        if log_header == Default::default() {
            return None;
        }

        return Some(log_header);
    }
}

impl Display for LogHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Whisky Version: {}\n", self.whisky_version.0)?;
        write!(f, "Date: {}\n", self.date)?;
        write!(f, "macOS Version: {}\n", self.macos_version.0)
    }
}
