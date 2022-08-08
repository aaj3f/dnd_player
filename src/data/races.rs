use std::{io, str::FromStr};

use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

use super::utils::{choose_value, pretty_print, Choosable, BLUE, PURPLE, RED};

use super::utils::StringJoin;

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
pub enum Race {
    Dwarf(Dwarf),
    Elf(Elf),
    Halfling(Halfling),
    Human(Human),
    Dragonborn,
    Gnome(Gnome),
    HalfElf,
    HalfOrc,
    Tiefling,
}

impl Race {
    pub fn choose_subrace(self) -> Self {
        let mut rng = rand::thread_rng();

        loop {
            let options: String = match self {
                Self::Dwarf(_) => Dwarf::join_string(),
                Self::Elf(_) => Elf::join_string(),
                Self::Halfling(_) => Halfling::join_string(),
                Self::Human(_) => Human::join_string(),
                Self::Gnome(_) => Gnome::join_string(),
                _ => panic!("How did you get here with a race that has no subrace!"),
            };
            pretty_print("Please choose from the following: ", BLUE, false);
            pretty_print(&format!("{}", options), PURPLE, true);
            pretty_print("(press ENTER to randomize):", PURPLE, false);
            let mut input_str = String::new();
            match io::stdin().read_line(&mut input_str) {
                Ok(length) => {
                    if length > 1 {
                        // let match_value = matcher(&input_str);
                        let match_result = match self {
                            Self::Dwarf(_) => {
                                match Dwarf::from_str(&input_str.trim().to_lowercase()) {
                                    Ok(v) => Self::Dwarf(v),
                                    Err(_) => {
                                        println!("Not an available option");
                                        continue;
                                    }
                                }
                            }
                            Self::Elf(_) => match Elf::from_str(&input_str.trim().to_lowercase()) {
                                Ok(v) => Self::Elf(v),
                                Err(_) => {
                                    println!("Not an available option");
                                    continue;
                                }
                            },
                            Self::Halfling(_) => {
                                match Halfling::from_str(&input_str.trim().to_lowercase()) {
                                    Ok(v) => Self::Halfling(v),
                                    Err(_) => {
                                        println!("Not an available option");
                                        continue;
                                    }
                                }
                            }
                            Self::Human(_) => {
                                match Human::from_str(&input_str.trim().to_lowercase()) {
                                    Ok(v) => Self::Human(v),
                                    Err(_) => {
                                        println!("Not an available option");
                                        continue;
                                    }
                                }
                            }
                            Self::Gnome(_) => {
                                match Gnome::from_str(&input_str.trim().to_lowercase()) {
                                    Ok(v) => Self::Gnome(v),
                                    Err(_) => {
                                        println!("Not an available option");
                                        continue;
                                    }
                                }
                            }
                            _ => break self,
                        };
                        break match_result;
                    } else {
                        break match self {
                            Self::Dwarf(_) => Self::Dwarf(Dwarf::iter().choose(&mut rng).unwrap()),
                            Self::Elf(_) => Self::Elf(Elf::iter().choose(&mut rng).unwrap()),
                            Self::Halfling(_) => {
                                Self::Halfling(Halfling::iter().choose(&mut rng).unwrap())
                            }
                            Self::Human(_) => Self::Human(Human::iter().choose(&mut rng).unwrap()),
                            Self::Gnome(_) => Self::Gnome(Gnome::iter().choose(&mut rng).unwrap()),
                            _ => self,
                        };
                    }
                }
                _ => {
                    pretty_print("UNACCEPTABLE", RED, true);
                    continue;
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
pub enum Dwarf {
    #[default]
    HillDwarf,
    MountainDwarf,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
pub enum Elf {
    #[default]
    DarkElf,
    HighElf,
    WoodElf,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
pub enum Halfling {
    #[default]
    Lightfoot,
    Stout,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
pub enum Human {
    #[default]
    Standard,
    Variant,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
pub enum Gnome {
    #[default]
    Forest,
    Rock,
}

impl Choosable<Race> for Race {
    fn choose() -> Race {
        choose_value("\nWhat is your character's race?", &Race::collect_string())
    }
}
