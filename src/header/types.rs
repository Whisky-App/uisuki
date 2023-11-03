use std::fmt::{Display, Formatter};
use semver::{BuildMetadata, Prerelease, Version};
use std::str::FromStr;

#[derive(Default, PartialEq)]
pub enum WinVersion {
    WinXP,
    Win7,
    Win8,
    Wine81,
    #[default]
    Win10,
}

impl Display for WinVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WinVersion::WinXP => write!(f, "Windows XP"),
            WinVersion::Win7 => write!(f, "Windows 7"),
            WinVersion::Win8 => write!(f, "Windows 8"),
            WinVersion::Wine81 => write!(f, "Windows 8.1"),
            WinVersion::Win10 => write!(f, "Windows 10"),
        }
    }
}

impl FromStr for WinVersion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

#[derive(Default, PartialEq)]
pub enum EnhancedSync {
    None,
    ESync,
    #[default]
    MSync,
}

impl Display for EnhancedSync {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EnhancedSync::None => write!(f, "None"),
            EnhancedSync::ESync => write!(f, "ESync"),
            EnhancedSync::MSync => write!(f, "MSync"),
        }
    }
}

impl FromStr for EnhancedSync {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(EnhancedSync::None),
            "esync" => Ok(EnhancedSync::ESync),
            "msync" => Ok(EnhancedSync::MSync),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq)]
pub struct WhiskyVersion(pub Version);
impl Default for WhiskyVersion {
    fn default() -> Self {
        return WhiskyVersion {
            0: Version {
                major: 0,
                minor: 0,
                patch: 0,
                pre: Prerelease::EMPTY,
                build: BuildMetadata::EMPTY,
            },
        };
    }
}
