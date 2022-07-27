use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum Race {
    Dwarf(Dwarf),
    Elf(Elf),
    Halfling(Halfling),
    Human(Human),
    Dragonborn,
    Gnome(Gnome),
    #[strum(serialize = "half elf", serialize = "halfelf", serialize = "half-elf")]
    HalfElf,
    #[strum(serialize = "half orc", serialize = "halforc", serialize = "half-orc")]
    HalfOrc,
    Tiefling,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
pub enum Dwarf {
    #[default]
    HillDwarf,
    MountainDwarf,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
pub enum Elf {
    #[default]
    DarkElf,
    HighElf,
    WoodElf,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
pub enum Halfling {
    #[default]
    Lightfoot,
    Stout,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
pub enum Human {
    #[default]
    Standard,
    Variant,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display, Default)]
pub enum Gnome {
    #[default]
    Forest,
    Rock,
}
