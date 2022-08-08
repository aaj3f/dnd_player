use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

use super::utils::{choose_value, Choosable};

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
        choose_value(
            "What is your character's class?",
            &Class::collect_string(),
            // class_match_string
        )
    }
}

impl Class {
    pub fn choose_subclass(self) -> Self {
        match self {
            Self::Artificer(_) => Self::Artificer(ArtificerSubclass::choose()),
            Self::Barbarian(_) => Self::Barbarian(BarbarianSubclass::choose()),
            Self::Bard(_) => Self::Bard(BardSubclass::choose()),
            Self::Cleric(_) => Self::Cleric(ClericSubclass::choose()),
            Self::Druid(_) => Self::Druid(DruidSubclass::choose()),
            Self::Fighter(_) => Self::Fighter(FighterSubclass::choose()),
            Self::Monk(_) => Self::Monk(MonkSubclass::choose()),
            Self::Paladin(_) => Self::Paladin(PaladinSubclass::choose()),
            Self::Ranger(_) => Self::Ranger(RangerSubclass::choose()),
            Self::Rogue(_) => Self::Rogue(RogueSubclass::choose()),
            Self::Sorcerer(_) => Self::Sorcerer(SorcererSubclass::choose()),
            Self::Warlock(_) => Self::Warlock(WarlockSubclass::choose()),
            Self::Wizard(_) => Self::Wizard(WizardSubclass::choose()),
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

impl Choosable<ArtificerSubclass> for ArtificerSubclass {
    fn choose() -> ArtificerSubclass {
        choose_value(
            "What is your Artificer's subclass?",
            &ArtificerSubclass::collect_string(),
            // class_match_string
        )
    }
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

impl Choosable<BarbarianSubclass> for BarbarianSubclass {
    fn choose() -> BarbarianSubclass {
        choose_value(
            "What is your Barbarian's subclass?",
            &BarbarianSubclass::collect_string(),
            // class_match_string
        )
    }
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

impl Choosable<BardSubclass> for BardSubclass {
    fn choose() -> BardSubclass {
        choose_value(
            "What is your Bard's subclass?",
            &BardSubclass::collect_string(),
            // class_match_string
        )
    }
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

impl Choosable<ClericSubclass> for ClericSubclass {
    fn choose() -> ClericSubclass {
        choose_value(
            "What is your Cleric's subclass?",
            &ClericSubclass::collect_string(),
            // class_match_string
        )
    }
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

impl Choosable<DruidSubclass> for DruidSubclass {
    fn choose() -> DruidSubclass {
        choose_value(
            "What is your Druid's subclass?",
            &DruidSubclass::collect_string(),
            // class_match_string
        )
    }
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

impl Choosable<FighterSubclass> for FighterSubclass {
    fn choose() -> FighterSubclass {
        choose_value(
            "What is your Fighter's subclass?",
            &FighterSubclass::collect_string(),
            // class_match_string
        )
    }
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

impl Choosable<MonkSubclass> for MonkSubclass {
    fn choose() -> MonkSubclass {
        choose_value(
            "What is your Monk's subclass?",
            &MonkSubclass::collect_string(),
            // class_match_string
        )
    }
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

impl Choosable<PaladinSubclass> for PaladinSubclass {
    fn choose() -> PaladinSubclass {
        choose_value(
            "What is your Paladin's subclass?",
            &PaladinSubclass::collect_string(),
            // class_match_string
        )
    }
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

impl Choosable<RangerSubclass> for RangerSubclass {
    fn choose() -> RangerSubclass {
        choose_value(
            "What is your Ranger's subclass?",
            &RangerSubclass::collect_string(),
            // class_match_string
        )
    }
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

impl Choosable<RogueSubclass> for RogueSubclass {
    fn choose() -> RogueSubclass {
        choose_value(
            "What is your Rogue's subclass?",
            &RogueSubclass::collect_string(),
            // class_match_string
        )
    }
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

impl Choosable<SorcererSubclass> for SorcererSubclass {
    fn choose() -> SorcererSubclass {
        choose_value(
            "What is your Sorcerer's subclass?",
            &SorcererSubclass::collect_string(),
            // class_match_string
        )
    }
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

impl Choosable<WarlockSubclass> for WarlockSubclass {
    fn choose() -> WarlockSubclass {
        choose_value(
            "What is your Warlock's subclass?",
            &WarlockSubclass::collect_string(),
            // class_match_string
        )
    }
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

impl Choosable<WizardSubclass> for WizardSubclass {
    fn choose() -> WizardSubclass {
        choose_value(
            "What is your Wizard's subclass?",
            &WizardSubclass::collect_string(),
            // class_match_string
        )
    }
}
