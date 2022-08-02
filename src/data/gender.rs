use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

use crate::{choose_value, Choosable};

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
            "\nWhat is your character's gender?",
            "Male, Female, None",
            // gender_match_string,
        )
    }
}
