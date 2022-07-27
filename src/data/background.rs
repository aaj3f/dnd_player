use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

use crate::{choose_value, Choosable};

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum Background {
    Acolyte,
    Charlatan,
    Criminal,
    Entertainer,
    #[strum(
        serialize = "folk hero",
        serialize = "folkhero",
        serialize = "folk-hero"
    )]
    FolkHero,
    #[strum(
        serialize = "guild artisan",
        serialize = "guildartisan",
        serialize = "guild-artisan"
    )]
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
            "Acolyte, Charlatan, Criminal, Entertainer, Folk Hero, Guild Artisan,\nHermit, Noble, Outlander, Sailor, Soldier, or Urchin",
            // background_match_string
        )
    }
}
