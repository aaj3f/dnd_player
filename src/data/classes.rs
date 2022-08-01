use rand::prelude::IteratorRandom;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::string::ToString;
use std::{
    io::{self},
    str::FromStr,
    thread, time,
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

use crate::{choose_value, pretty_print, Choosable, BLUE, PURPLE, RED};

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum Class {
    Artificer(Option<ArtificerSubclass>),
    Barbarian(Option<BarbarianSubclass>),
    Bard(Option<BardSubclass>),
    Cleric(Option<ClericSubclass>),
    Druid(Option<DruidSubclass>),
    Fighter(Option<FighterSubclass>),
    Monk(Option<MonkSubclass>),
    Paladin(Option<PaladinSubclass>),
    Ranger(Option<RangerSubclass>),
    Rogue(Option<RogueSubclass>),
    Sorcerer(Option<SorcererSubclass>),
    Warlock(Option<WarlockSubclass>),
    Wizard(Option<WizardSubclass>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum Subclass {
    ArtificerSubclass,
    BarbarianSubclass,
    BardSubclass,
    ClericSubclass,
    DruidSubclass,
    FighterSubclass,
    MonkSubclass,
    PaladinSubclass,
    RangerSubclass,
    RogueSubclass,
    SorcererSubclass,
    WarlockSubclass,
    WizardSubclass,
}

pub trait StringJoin<T> {
    fn join_string() -> String;
}

impl<T> StringJoin<T> for T
where
    T: Display + IntoEnumIterator,
{
    fn join_string() -> String {
        T::iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

impl Choosable<Class> for Class {
    fn choose() -> Class {
        choose_value("\nWhat is your character's class?", 
        "Artificer, Barbarian, Bard, Cleric, Druid, Figher, Monk, Paladin, Ranger, Rogue, Sorcerer, Warlock, Wizard", 
        // class_match_string
    )
    }
}

impl Class {
    pub fn choose_subclass(self) -> Self {
        let one_second = time::Duration::from_secs(1);
        let mut rng = rand::thread_rng();

        let should_continue = loop {
            pretty_print(
            "\nYour character level is high enough to choose a sub-class.\nWould you like to go ahead and choose a sub-class for your character? [Y/N]: ",
            BLUE,
            false,
        );

            let mut answer = String::new();
            match io::stdin().read_line(&mut answer) {
                Ok(length) => {
                    if length > 1 {
                        match answer.trim().to_lowercase().as_str() {
                            "yes" | "y" => break true,
                            "no" | "n" => break false,
                            _ => {
                                continue;
                            }
                        }
                    } else {
                        break true;
                    }
                }
                _ => {
                    pretty_print("ERROR, please try again", RED, true);
                    continue;
                }
            }
        };

        if !should_continue {
            return self;
        }

        loop {
            let options: String = match self {
                Self::Artificer(_) => ArtificerSubclass::join_string(),
                Self::Barbarian(_) => BarbarianSubclass::join_string(),
                Self::Bard(_) => BardSubclass::join_string(),
                Self::Cleric(_) => ClericSubclass::join_string(),
                Self::Druid(_) => DruidSubclass::join_string(),
                Self::Fighter(_) => FighterSubclass::join_string(),
                Self::Monk(_) => MonkSubclass::join_string(),
                Self::Paladin(_) => PaladinSubclass::join_string(),
                Self::Ranger(_) => RangerSubclass::join_string(),
                Self::Rogue(_) => RogueSubclass::join_string(),
                Self::Sorcerer(_) => SorcererSubclass::join_string(),
                Self::Warlock(_) => WarlockSubclass::join_string(),
                Self::Wizard(_) => WizardSubclass::join_string(),
            };

            pretty_print("Please choose from the following: ", BLUE, false);
            pretty_print(&format!("{}", options), PURPLE, true);
            pretty_print("(press ENTER to randomize):", PURPLE, false);
            thread::sleep(one_second);
            let mut input_str = String::new();
            match io::stdin().read_line(&mut input_str) {
                Ok(length) => {
                    if length > 1 {
                        // let match_value = matcher(&input_str);
                        let match_result = match self {
                            Self::Artificer(_) => {
                                match ArtificerSubclass::from_str(&input_str.trim()) {
                                    Ok(v) => Self::Artificer(Some(v)),
                                    Err(_) => {
                                        println!("Not an available option");
                                        continue;
                                    }
                                }
                            }
                            Self::Barbarian(_) => {
                                match BarbarianSubclass::from_str(&input_str.trim()) {
                                    Ok(v) => Self::Barbarian(Some(v)),
                                    Err(_) => {
                                        println!("Not an available option");
                                        continue;
                                    }
                                }
                            }
                            Self::Bard(_) => match BardSubclass::from_str(&input_str.trim()) {
                                Ok(v) => Self::Bard(Some(v)),
                                Err(_) => {
                                    println!("Not an available option");
                                    continue;
                                }
                            },
                            Self::Cleric(_) => match ClericSubclass::from_str(&input_str.trim()) {
                                Ok(v) => Self::Cleric(Some(v)),
                                Err(_) => {
                                    println!("Not an available option");
                                    continue;
                                }
                            },
                            Self::Druid(_) => match DruidSubclass::from_str(&input_str.trim()) {
                                Ok(v) => Self::Druid(Some(v)),
                                Err(_) => {
                                    println!("Not an available option");
                                    continue;
                                }
                            },
                            Self::Fighter(_) => {
                                match FighterSubclass::from_str(&input_str.trim()) {
                                    Ok(v) => Self::Fighter(Some(v)),
                                    Err(_) => {
                                        println!("Not an available option");
                                        continue;
                                    }
                                }
                            }
                            Self::Monk(_) => match MonkSubclass::from_str(&input_str.trim()) {
                                Ok(v) => Self::Monk(Some(v)),
                                Err(_) => {
                                    println!("Not an available option");
                                    continue;
                                }
                            },
                            Self::Paladin(_) => {
                                match PaladinSubclass::from_str(&input_str.trim()) {
                                    Ok(v) => Self::Paladin(Some(v)),
                                    Err(_) => {
                                        println!("Not an available option");
                                        continue;
                                    }
                                }
                            }
                            Self::Ranger(_) => match RangerSubclass::from_str(&input_str.trim()) {
                                Ok(v) => Self::Ranger(Some(v)),
                                Err(_) => {
                                    println!("Not an available option");
                                    continue;
                                }
                            },
                            Self::Rogue(_) => match RogueSubclass::from_str(&input_str.trim()) {
                                Ok(v) => Self::Rogue(Some(v)),
                                Err(_) => {
                                    println!("Not an available option");
                                    continue;
                                }
                            },
                            Self::Sorcerer(_) => {
                                match SorcererSubclass::from_str(&input_str.trim()) {
                                    Ok(v) => Self::Sorcerer(Some(v)),
                                    Err(_) => {
                                        println!("Not an available option");
                                        continue;
                                    }
                                }
                            }
                            Self::Warlock(_) => {
                                match WarlockSubclass::from_str(&input_str.trim()) {
                                    Ok(v) => Self::Warlock(Some(v)),
                                    Err(_) => {
                                        println!("Not an available option");
                                        continue;
                                    }
                                }
                            }
                            Self::Wizard(_) => match WizardSubclass::from_str(&input_str.trim()) {
                                Ok(v) => Self::Wizard(Some(v)),
                                Err(_) => {
                                    println!("Not an available option");
                                    continue;
                                }
                            },
                        };
                        break match_result;
                    } else {
                        break match self {
                            Self::Artificer(_) => Self::Artificer(Some(
                                ArtificerSubclass::iter().choose(&mut rng).unwrap(),
                            )),
                            Self::Barbarian(_) => Self::Barbarian(Some(
                                BarbarianSubclass::iter().choose(&mut rng).unwrap(),
                            )),
                            Self::Bard(_) => {
                                Self::Bard(Some(BardSubclass::iter().choose(&mut rng).unwrap()))
                            }
                            Self::Cleric(_) => {
                                Self::Cleric(Some(ClericSubclass::iter().choose(&mut rng).unwrap()))
                            }
                            Self::Druid(_) => {
                                Self::Druid(Some(DruidSubclass::iter().choose(&mut rng).unwrap()))
                            }
                            Self::Fighter(_) => Self::Fighter(Some(
                                FighterSubclass::iter().choose(&mut rng).unwrap(),
                            )),
                            Self::Monk(_) => {
                                Self::Monk(Some(MonkSubclass::iter().choose(&mut rng).unwrap()))
                            }
                            Self::Paladin(_) => Self::Paladin(Some(
                                PaladinSubclass::iter().choose(&mut rng).unwrap(),
                            )),
                            Self::Ranger(_) => {
                                Self::Ranger(Some(RangerSubclass::iter().choose(&mut rng).unwrap()))
                            }
                            Self::Rogue(_) => {
                                Self::Rogue(Some(RogueSubclass::iter().choose(&mut rng).unwrap()))
                            }
                            Self::Sorcerer(_) => Self::Sorcerer(Some(
                                SorcererSubclass::iter().choose(&mut rng).unwrap(),
                            )),
                            Self::Warlock(_) => Self::Warlock(Some(
                                WarlockSubclass::iter().choose(&mut rng).unwrap(),
                            )),
                            Self::Wizard(_) => {
                                Self::Wizard(Some(WizardSubclass::iter().choose(&mut rng).unwrap()))
                            }
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
pub enum ArtificerSubclass {
    #[default]
    Alchemist,
    Artillerist,
    BattleSmith,
    Armorer,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
pub enum BarbarianSubclass {
    #[default]
    Berserker,
    Totem,
    AncestralGuardian,
    StormHerald,
    Zealot,
    Beast,
    WildMagic,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
pub enum BardSubclass {
    #[default]
    Lore,
    Valor,
    Glamour,
    Swords,
    Whispers,
    Eloquence,
    Creation,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
pub enum ClericSubclass {
    #[default]
    Knowledge,
    Life,
    Light,
    Nature,
    Tempest,
    Trickery,
    War,
    Forge,
    Grave,
    Order,
    Peace,
    Twilight,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
pub enum DruidSubclass {
    #[default]
    Land,
    Moon,
    Dreams,
    Shepherd,
    Spores,
    Stars,
    Wildfire,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
pub enum FighterSubclass {
    #[default]
    Champion,
    BattleMaster,
    EldritchKnight,
    ArcaneArcher,
    Cavalier,
    PsiWarrior,
    RuneKnight,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
pub enum MonkSubclass {
    #[default]
    OpenHand,
    Shadow,
    FourElements,
    SunSoul,
    Drunken,
    Kensei,
    Mercy,
    AstralSelf,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
pub enum PaladinSubclass {
    #[default]
    Devotion,
    Ancients,
    Vengeance,
    Conquest,
    Redemption,
    Glory,
    Watchers,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
pub enum RangerSubclass {
    #[default]
    Hunter,
    BeastMaster,
    GloomStalker,
    HorizonWalker,
    MonsterSlayer,
    FeyWanderer,
    Swarmkeeper,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
pub enum RogueSubclass {
    #[default]
    Thief,
    Assassin,
    Mastermind,
    Swashbuckler,
    ArcaneTrickster,
    Inquisitive,
    Scout,
    Phantom,
    Soulknife,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
pub enum SorcererSubclass {
    #[default]
    Draconic,
    WildMagic,
    Storm,
    DivineSoul,
    ShadowMagic,
    AberrantMind,
    ClockworkSoul,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
pub enum WarlockSubclass {
    #[default]
    Archfey,
    Fiend,
    GreatOldOne,
    Celestial,
    Hexblade,
    Fathomless,
    Genie,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
pub enum WizardSubclass {
    #[default]
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
    Bladesinger,
    WarMagic,
}
