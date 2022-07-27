use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

use crate::{choose_value, Choosable};

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

impl Choosable<Class> for Class {
    fn choose() -> Class {
        choose_value("\nWhat is your character's class?", 
        "Artificer, Barbarian, Bard, Cleric, Druid, Figher, Monk, Paladin, Ranger, Rogue, Sorcerer, Warlock, Wizard", 
        // class_match_string
    )
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
#[strum(ascii_case_insensitive)]
pub enum ArtificerSubclass {
    #[default]
    Alchemist,
    Artillerist,
    BattleSmith,
    Armorer,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
#[strum(ascii_case_insensitive)]
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
#[strum(ascii_case_insensitive)]
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
#[strum(ascii_case_insensitive)]
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
#[strum(ascii_case_insensitive)]
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
#[strum(ascii_case_insensitive)]
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
#[strum(ascii_case_insensitive)]
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
#[strum(ascii_case_insensitive)]
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
#[strum(ascii_case_insensitive)]
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
#[strum(ascii_case_insensitive)]
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
#[strum(ascii_case_insensitive)]
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
#[strum(ascii_case_insensitive)]
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
#[strum(ascii_case_insensitive)]
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
