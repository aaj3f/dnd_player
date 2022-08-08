use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

use super::utils::{choose_value, Choosable, StringJoin};

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
pub enum Background {
    Acolyte,
    Charlatan,
    Criminal,
    Entertainer,
    FolkHero,
    GuildArtisan,
    Hermit,
    Noble,
    Outlander,
    Sailor,
    Soldier,
    Urchin,
}

impl Choosable<Background> for Background {
    fn choose() -> Background {
        choose_value(
            "\nWhat is your character's background?",
            &Background::collect_string(), // background_match_string
        )
    }
}
