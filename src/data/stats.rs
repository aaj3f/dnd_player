use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use strum_macros::{Display, EnumIter, EnumString};

use super::utils::StringJoin;

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display)]
pub enum Stat {
    Str(u8),
    Dex(u8),
    Con(u8),
    Int(u8),
    Wis(u8),
    Chr(u8),
}

impl Stat {
    pub fn get_modifier(&self) -> i8 {
        let stat_val = match *self {
            Stat::Str(val) => val,
            Stat::Dex(val) => val,
            Stat::Con(val) => val,
            Stat::Int(val) => val,
            Stat::Wis(val) => val,
            Stat::Chr(val) => val,
        };
        match stat_val {
            1 => -5,
            2 | 3 => -4,
            4 | 5 => -3,
            6 | 7 => -2,
            8 | 9 => -1,
            10 | 11 => 0,
            12 | 13 => 1,
            14 | 15 => 2,
            16 | 17 => 3,
            18 | 19 => 4,
            20 | 21 => 5,
            22 | 23 => 6,
            24 | 25 => 7,
            26 | 27 => 8,
            28 | 29 => 9,
            30 => 10,
            _ => 0,
        }
    }
    pub fn display(&self) -> String {
        let stat_modifier = self.get_modifier();
        let sign = match stat_modifier.cmp(&0) {
            Ordering::Greater => "+",
            _ => "",
        };
        format!("{}{}", sign, stat_modifier)
    }

    pub fn show_name(&self) -> &str {
        match &self {
            Stat::Str(_) => "Strength",
            Stat::Dex(_) => "Dexterity",
            Stat::Con(_) => "Constitution",
            Stat::Int(_) => "Intelligence",
            Stat::Wis(_) => "Wisdom",
            Stat::Chr(_) => "Charisma",
        }
    }

    pub fn list() -> String {
        Stat::join_string()
    }
}
