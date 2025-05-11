use crate::library::colors::transforms::{DevanoPalette, NeutralHexes, AccentHexes};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug)]
pub enum NamedPalette {
    Default,
    Lime,
}

impl NamedPalette {
    pub fn as_str(&self) -> &'static str {
        match self {
            NamedPalette::Default => "default",
            NamedPalette::Lime => "lime",
        }
    }
    // Get the corresponding Description for the enum variant
    pub fn description(&self) -> &'static str {
        match self {
            NamedPalette::Default => "Neutral neutrals. Icy cool & saturated accents.",
            NamedPalette::Lime => "Neutral neutrals. Kelly/Forest/Cyan accents",
        }
    }

    /// Get the corresponding `DevanoPalette` for the enum variant
    pub fn get_palette(&self) -> DevanoPalette {
        match self {
            NamedPalette::Default => DevanoPalette {
                kora: NeutralHexes {
                    ara: "#030101".to_string(),
                    ene: "#1d1A17".to_string(),
                    izi: "#33312F".to_string(),
                    ona: "#4A4948".to_string(),
                },
                aleva: NeutralHexes {
                    ara: "#B6B2AF".to_string(),
                    ene: "#D9D4D0".to_string(),
                    izi: "#EBE6E1".to_string(),
                    ona: "#FDF7F2".to_string(),
                },
                ara: AccentHexes {
                    ara: "#84d9b6".to_string(),
                    ene: "#b8ebd8".to_string(),
                    izi: "#DEFCF4".to_string(),
                },
                ene: AccentHexes {
                    ara: "#78D8DD".to_string(),
                    ene: "#B0EBED".to_string(),
                    izi: "#D7FCFC".to_string(),
                },
                izi: AccentHexes {
                    ara: "#81D0FD".to_string(),
                    ene: "#B6E6FE".to_string(),
                    izi: "#DCFAFF".to_string(),
                },
                ona: AccentHexes {
                    ara: "#082B1D".to_string(),
                    ene: "#3F6E5E".to_string(),
                    izi: "#57827E".to_string(),
                },
                uvo: AccentHexes {
                    ara: "#04282D".to_string(),
                    ene: "#366E70".to_string(),
                    izi: "#4B9295".to_string(),
                },
                bala: AccentHexes {
                    ara: "#072738".to_string(),
                    ene: "#3D6A7E".to_string(),
                    izi: "#568DA5".to_string(),
                },
            },
            NamedPalette::Lime => DevanoPalette {
                kora: NeutralHexes {
                    ara: "#030303".to_string(),
                    ene: "#151515".to_string(),
                    izi: "#2B2B2B".to_string(),
                    ona: "#444444".to_string(),
                },
                aleva: NeutralHexes {
                    ara: "#D4D4D4".to_string(),
                    ene: "#DFDFDF".to_string(),
                    izi: "#E9E9E9".to_string(),
                    ona: "#FAFAFA".to_string(),
                },
                ara: AccentHexes {
                    ara: "#4CFF03".to_string(),
                    ene: "#8EFF30".to_string(),
                    izi: "#B5FF44".to_string(),
                },
                ene: AccentHexes {
                    ara: "#00FF7E".to_string(),
                    ene: "#00FFB4".to_string(),
                    izi: "#00FFDA".to_string(),
                },
                izi: AccentHexes {
                    ara: "#00FFFF".to_string(),
                    ene: "#00FFFF".to_string(),
                    izi: "#00FFFF".to_string(),
                },
                ona: AccentHexes {
                    ara: "#003900".to_string(),
                    ene: "#007C00".to_string(),
                    izi: "#00A300".to_string(),
                },
                uvo: AccentHexes {
                    ara: "#004607".to_string(),
                    ene: "#00893C".to_string(),
                    izi: "#00B053".to_string(),
                },
                bala: AccentHexes {
                    ara: "#00434E".to_string(),
                    ene: "#00858F".to_string(),
                    izi: "#00ADB7".to_string(),
                },
            },
        }
    }

    /// Convert a string to a `NamedPalette` variant
    pub fn from_str(name: &str) -> Option<Self> {
        NamedPalette::iter().find(|variant| variant.as_str() == name)
    }
}