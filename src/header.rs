pub use crate::header::types::{EnhancedSync, WhiskyVersion, WinVersion};
use iso8601::Date;
use std::fmt::{Display, Formatter};

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

impl Display for LogHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Whisky Version: {}\n", self.whisky_version.0)?;
        write!(f, "Date: {}\n", self.date)?;
        write!(f, "macOS Version: {}\n", self.macos_version.0)
    }
}
