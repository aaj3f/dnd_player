use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Stat {
    Str(i8),
    Dex(i8),
    Con(i8),
    Int(i8),
    Wis(i8),
    Chr(i8),
}

impl Stat {
    pub fn display(&self) -> String {
        let stat_val = match *self {
            Stat::Str(val) => val,
            Stat::Dex(val) => val,
            Stat::Con(val) => val,
            Stat::Int(val) => val,
            Stat::Wis(val) => val,
            Stat::Chr(val) => val,
        };
        let stat_modifier: i8 = match stat_val {
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
        };
        let sign = match stat_modifier.cmp(&0) {
            Ordering::Greater => "+",
            _ => "",
        };
        format!("{}{}", sign, stat_modifier)
    }
}
