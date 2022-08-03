use rand::prelude::IteratorRandom;
use serde::{Deserialize, Serialize};
use std::{
    io::{self},
    str::FromStr,
    thread, time,
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

use super::utils::{choose_value, pretty_print, Choosable, BLUE, PURPLE, RED};

use super::utils::StringJoin;

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum Class {
    Artificer(ArtificerSubclass),
    Barbarian(BarbarianSubclass),
    Bard(BardSubclass),
    Cleric(ClericSubclass),
    Druid(DruidSubclass),
    Fighter(FighterSubclass),
    Monk(MonkSubclass),
    Paladin(PaladinSubclass),
    Ranger(RangerSubclass),
    Rogue(RogueSubclass),
    Sorcerer(SorcererSubclass),
    Warlock(WarlockSubclass),
    Wizard(WizardSubclass),
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

pub trait HasSubclass<T> {
    fn get_subclass(&self) -> String;
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
                                    Ok(v) => Self::Artificer(v),
                                    Err(_) => {
                                        println!("Not an available option");
                                        continue;
                                    }
                                }
                            }
                            Self::Barbarian(_) => {
                                match BarbarianSubclass::from_str(&input_str.trim()) {
                                    Ok(v) => Self::Barbarian(v),
                                    Err(_) => {
                                        println!("Not an available option");
                                        continue;
                                    }
                                }
                            }
                            Self::Bard(_) => match BardSubclass::from_str(&input_str.trim()) {
                                Ok(v) => Self::Bard(v),
                                Err(_) => {
                                    println!("Not an available option");
                                    continue;
                                }
                            },
                            Self::Cleric(_) => match ClericSubclass::from_str(&input_str.trim()) {
                                Ok(v) => Self::Cleric(v),
                                Err(_) => {
                                    println!("Not an available option");
                                    continue;
                                }
                            },
                            Self::Druid(_) => match DruidSubclass::from_str(&input_str.trim()) {
                                Ok(v) => Self::Druid(v),
                                Err(_) => {
                                    println!("Not an available option");
                                    continue;
                                }
                            },
                            Self::Fighter(_) => {
                                match FighterSubclass::from_str(&input_str.trim()) {
                                    Ok(v) => Self::Fighter(v),
                                    Err(_) => {
                                        println!("Not an available option");
                                        continue;
                                    }
                                }
                            }
                            Self::Monk(_) => match MonkSubclass::from_str(&input_str.trim()) {
                                Ok(v) => Self::Monk(v),
                                Err(_) => {
                                    println!("Not an available option");
                                    continue;
                                }
                            },
                            Self::Paladin(_) => {
                                match PaladinSubclass::from_str(&input_str.trim()) {
                                    Ok(v) => Self::Paladin(v),
                                    Err(_) => {
                                        println!("Not an available option");
                                        continue;
                                    }
                                }
                            }
                            Self::Ranger(_) => match RangerSubclass::from_str(&input_str.trim()) {
                                Ok(v) => Self::Ranger(v),
                                Err(_) => {
                                    println!("Not an available option");
                                    continue;
                                }
                            },
                            Self::Rogue(_) => match RogueSubclass::from_str(&input_str.trim()) {
                                Ok(v) => Self::Rogue(v),
                                Err(_) => {
                                    println!("Not an available option");
                                    continue;
                                }
                            },
                            Self::Sorcerer(_) => {
                                match SorcererSubclass::from_str(&input_str.trim()) {
                                    Ok(v) => Self::Sorcerer(v),
                                    Err(_) => {
                                        println!("Not an available option");
                                        continue;
                                    }
                                }
                            }
                            Self::Warlock(_) => {
                                match WarlockSubclass::from_str(&input_str.trim()) {
                                    Ok(v) => Self::Warlock(v),
                                    Err(_) => {
                                        println!("Not an available option");
                                        continue;
                                    }
                                }
                            }
                            Self::Wizard(_) => match WizardSubclass::from_str(&input_str.trim()) {
                                Ok(v) => Self::Wizard(v),
                                Err(_) => {
                                    println!("Not an available option");
                                    continue;
                                }
                            },
                        };
                        break match_result;
                    } else {
                        break match self {
                            Self::Artificer(_) => {
                                Self::Artificer(ArtificerSubclass::iter().choose(&mut rng).unwrap())
                            }
                            Self::Barbarian(_) => {
                                Self::Barbarian(BarbarianSubclass::iter().choose(&mut rng).unwrap())
                            }
                            Self::Bard(_) => {
                                Self::Bard(BardSubclass::iter().choose(&mut rng).unwrap())
                            }
                            Self::Cleric(_) => {
                                Self::Cleric(ClericSubclass::iter().choose(&mut rng).unwrap())
                            }
                            Self::Druid(_) => {
                                Self::Druid(DruidSubclass::iter().choose(&mut rng).unwrap())
                            }
                            Self::Fighter(_) => {
                                Self::Fighter(FighterSubclass::iter().choose(&mut rng).unwrap())
                            }
                            Self::Monk(_) => {
                                Self::Monk(MonkSubclass::iter().choose(&mut rng).unwrap())
                            }
                            Self::Paladin(_) => {
                                Self::Paladin(PaladinSubclass::iter().choose(&mut rng).unwrap())
                            }
                            Self::Ranger(_) => {
                                Self::Ranger(RangerSubclass::iter().choose(&mut rng).unwrap())
                            }
                            Self::Rogue(_) => {
                                Self::Rogue(RogueSubclass::iter().choose(&mut rng).unwrap())
                            }
                            Self::Sorcerer(_) => {
                                Self::Sorcerer(SorcererSubclass::iter().choose(&mut rng).unwrap())
                            }
                            Self::Warlock(_) => {
                                Self::Warlock(WarlockSubclass::iter().choose(&mut rng).unwrap())
                            }
                            Self::Wizard(_) => {
                                Self::Wizard(WizardSubclass::iter().choose(&mut rng).unwrap())
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
