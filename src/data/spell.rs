use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Spell {
    #[serde(rename = "_id")]
    id: i64,
    pub level: u8,
    pub name: String,
}

impl fmt::Display for Spell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level = match self.level {
            0 => String::from("Cantrip"),
            1 => format!("{}st", self.level),
            2 => format!("{}nd", self.level),
            3 => format!("{}rd", self.level),
            _ => format!("{}th", self.level),
        };
        write!(f, "{} [{}]", self.name, level)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SpellSlot {
    pub level: u8,
    pub available: bool,
}

impl SpellSlot {
    fn from_u8(level: u8) -> SpellSlot {
        SpellSlot {
            level,
            available: true,
        }
    }
    pub fn from_vec(vector: Vec<u8>) -> Vec<SpellSlot> {
        vector
            .into_iter()
            .map(|level| SpellSlot::from_u8(level))
            .collect()
    }
}
