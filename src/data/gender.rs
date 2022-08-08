use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

use super::utils::{choose_value, Choosable, StringJoin};

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum Gender {
    Male,
    Female,
    None,
}

impl Choosable<Gender> for Gender {
    fn choose() -> Gender {
        choose_value(
            "What is your character's gender?",
            &Gender::collect_string(), // gender_match_string,
        )
    }
}
