mod data;

use chrono::prelude::*;
use crossterm::{
    event, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
};
use data::classes::{Class, MonkClass};
use data::names;
use rand::prelude::*;
use rand::seq::SliceRandom;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use serde_yaml;
use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fmt::Error,
    fs,
    io::{self},
    path,
    str::FromStr,
    thread, time,
};
use std::{fmt::Display, io::prelude::*};
use strum::{IntoEnumIterator, ParseError};
use strum_macros::{Display, EnumIter, EnumString};
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

type MatchResult<T> = Result<T, ParseError>;

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display)]
#[strum(ascii_case_insensitive)]
enum Race {
    Dwarf,
    Elf,
    Halfling,
    Human,
    Dragonborn,
    Gnome,
    #[strum(serialize = "half elf", serialize = "halfelf", serialize = "half-elf")]
    HalfElf,
    #[strum(serialize = "half orc", serialize = "halforc", serialize = "half-orc")]
    HalfOrc,
    Tiefling,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display)]
#[strum(ascii_case_insensitive)]
enum Gender {
    Male,
    Female,
    #[strum(
        serialize = "non binary",
        serialize = "nonbinary",
        serialize = "non-binary"
    )]
    NonBinary,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Stat {
    Str(i8),
    Dex(i8),
    Con(i8),
    Int(i8),
    Wis(i8),
    Chr(i8),
}

impl Stat {
    fn display(&self) -> String {
        let stat_val = match *self {
            Stat::Str(val) => val,
            Stat::Dex(val) => val,
            Stat::Con(val) => val,
            Stat::Int(val) => val,
            Stat::Wis(val) => val,
            Stat::Chr(val) => val,
        };
        let stat_modifier: i8 = match stat_val {
            1 => -5,
            2 | 3 => -4,
            4 | 5 => -3,
            6 | 7 => -2,
            8 | 9 => -1,
            10 | 11 => 0,
            12 | 13 => 1,
            14 | 15 => 2,
            16 | 17 => 3,
            18 | 19 => 4,
            20 | 21 => 5,
            22 | 23 => 6,
            24 | 25 => 7,
            26 | 27 => 8,
            28 | 29 => 9,
            30 => 10,
            _ => 0,
        };
        let sign = match stat_modifier.cmp(&0) {
            Ordering::Greater => "+",
            _ => "",
        };
        format!("{}{}", sign, stat_modifier)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display)]
#[strum(ascii_case_insensitive)]
enum Background {
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Condition {
    None,
    Poisoned,
    //TODO: Continue to fill out
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Dice {
    D4,
    D8,
    D10,
    D12,
    D20,
    D100,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Status {
    armor_class: u8,
    conditions: Condition,
    blessed: bool,
    initiative: u8,
    hit_dice: Dice,
    current_hp: u8,
    maximum_hp: u8,
    speed: u16,
}

pub trait Choosable<T> {
    fn choose() -> T;
}

impl Choosable<Race> for Race {
    fn choose() -> Race {
        choose_value(
            "\nWhat is your character's race?",
            "Dwarf, Elf, Halfling, Human, Dragonborn, Gnome, Half-Elf, Half-Orc, or Tiefling",
            // race_match_string,
        )
    }
}

impl Choosable<Gender> for Gender {
    fn choose() -> Gender {
        choose_value(
            "\nWhat is your character's gender?",
            "Male, Female, Non-Binary, None",
            // gender_match_string,
        )
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

impl Choosable<Background> for Background {
    fn choose() -> Background {
        choose_value(
            "\nWhat is your character's background?",
            "Acolyte, Charlatan, Criminal, Entertainer, Folk Hero, Guild Artisan,\nHermit, Noble, Outlander, Sailor, Soldier, or Urchin",
            // background_match_string
        )
    }
}

impl Status {
    pub fn new(_stats: &[Stat]) -> Status {
        Status {
            armor_class: 12,
            conditions: Condition::None,
            blessed: false,
            initiative: 4,
            hit_dice: Dice::D8,
            current_hp: 10,
            maximum_hp: 10,
            speed: 30,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Character {
    name: String,
    level: u8,
    background: Background,
    race: Race,
    class: Class,
    stats: [Stat; 6],
    status: Status,
    gender: Gender,
}

impl Character {
    pub fn display(&self) {
        let mut table = Table::new();
        // table.max_column_width = 40;

        table.style = TableStyle::extended();

        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            "Character Sheet",
            12,
            Alignment::Center,
        )]));

        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Name", 1, Alignment::Center),
            TableCell::new_with_alignment(format!("{}", self.name), 12, Alignment::Center),
        ]));

        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Gender", 1, Alignment::Center),
            TableCell::new_with_alignment(format!("{}", self.gender), 12, Alignment::Center),
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Race", 1, Alignment::Center),
            TableCell::new_with_alignment(format!("{}", self.race), 12, Alignment::Center),
        ]));

        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Class", 1, Alignment::Center),
            TableCell::new_with_alignment(format!("{}", self.class), 12, Alignment::Center),
        ]));

        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Level", 1, Alignment::Center),
            TableCell::new_with_alignment(format!("{}", self.level), 12, Alignment::Center),
        ]));

        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Background", 1, Alignment::Center),
            TableCell::new_with_alignment(format!("{}", self.background), 12, Alignment::Center),
        ]));

        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Character", 5, Alignment::Center),
            TableCell::new(""),
            TableCell::new_with_alignment("Stats", 6, Alignment::Center),
        ]));

        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("HP", 1, Alignment::Left),
            TableCell::new_with_alignment("AC", 1, Alignment::Left),
            TableCell::new_with_alignment("Speed", 1, Alignment::Left),
            TableCell::new_with_alignment("Initiative", 1, Alignment::Left),
            TableCell::new_with_alignment("Blessed", 1, Alignment::Left),
            TableCell::new(""),
            TableCell::new_with_alignment("STR", 1, Alignment::Right),
            TableCell::new_with_alignment("DEX", 1, Alignment::Right),
            TableCell::new_with_alignment("CON", 1, Alignment::Right),
            TableCell::new_with_alignment("INT", 1, Alignment::Right),
            TableCell::new_with_alignment("WIS", 1, Alignment::Right),
            TableCell::new_with_alignment("CHR", 1, Alignment::Right),
        ]));

        let blessed_string = match self.status.blessed {
            true => "+",
            false => "-",
        };

        let stats_array: Vec<String> = self.stats.iter().map(|x| x.display()).collect();

        table.add_row(Row::new(vec![
            TableCell::new_with_alignment(
                format!("{}/{}", self.status.current_hp, self.status.maximum_hp),
                1,
                Alignment::Center,
            ),
            TableCell::new_with_alignment(
                format!("{}", self.status.armor_class),
                1,
                Alignment::Center,
            ),
            TableCell::new_with_alignment(format!("{}", self.status.speed), 1, Alignment::Center),
            TableCell::new_with_alignment(
                format!("{}", self.status.initiative),
                1,
                Alignment::Center,
            ),
            TableCell::new_with_alignment(format!("{}", blessed_string), 1, Alignment::Center),
            TableCell::new(""),
            TableCell::new_with_alignment(&stats_array[0], 1, Alignment::Center),
            TableCell::new_with_alignment(&stats_array[1], 1, Alignment::Center),
            TableCell::new_with_alignment(&stats_array[2], 1, Alignment::Center),
            TableCell::new_with_alignment(&stats_array[3], 1, Alignment::Center),
            TableCell::new_with_alignment(&stats_array[4], 1, Alignment::Center),
            TableCell::new_with_alignment(&stats_array[5], 1, Alignment::Center),
        ]));

        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            format!("Status Conditions: None"),
            12,
            Alignment::Left,
        )]));

        println!("{}", table.render());
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PlayObject {
    character: Character,
    #[serde(deserialize_with = "de_created_at", default = "empty_datetime")]
    created_at: DateTime<Utc>,
    #[serde(deserialize_with = "de_updated_at", default = "empty_updated_at")]
    updated_at: Option<DateTime<Utc>>,
    #[serde(skip_deserializing, default = "empty_datetime")]
    last_played_at: DateTime<Utc>,
}

fn empty_datetime() -> DateTime<Utc> {
    Utc::now()
}

fn empty_updated_at() -> Option<DateTime<Utc>> {
    None
}

fn de_created_at<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match Utc.datetime_from_str(&s, "%a %b %e %T %Y") {
        Ok(utc) => Ok(utc),
        Err(_) => Ok(Utc::now()),
    }
}

fn de_updated_at<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match Utc.datetime_from_str(&s, "%a %b %e %T %Y") {
        Ok(utc) => Ok(Some(utc)),
        Err(_) => Ok(None),
    }
}

// impl PlayObject {
//     fn new<T>(serde_object: T) -> PlayObject
//     where
//         T: DeserializeOwned,
//     {
//         let mut map = BTreeMap::new();

//     }
// }

const PURPLE: Color = Color::Rgb {
    r: 183,
    g: 117,
    b: 214,
};

const BLUE: Color = Color::Rgb {
    r: 19,
    g: 198,
    b: 255,
};

const RED: Color = Color::Rgb {
    r: 244,
    g: 67,
    b: 54,
};

fn race_match_string(s: &str) -> Result<Race, Error> {
    let normalized_string = s.trim().to_lowercase();
    match normalized_string.as_str() {
        "dwarf" => Ok(Race::Dwarf),
        "elf" => Ok(Race::Elf),
        "halfling" => Ok(Race::Halfling),
        "human" => Ok(Race::Human),
        "dragonborn" => Ok(Race::Dragonborn),
        "gnome" => Ok(Race::Gnome),
        "halfelf" | "half-elf" | "half elf" => Ok(Race::HalfElf),
        "halforc" | "half-orc" | "half orc" => Ok(Race::HalfOrc),
        "tiefling" => Ok(Race::Tiefling),
        _ => Err(Error),
    }
}

fn gender_match_string(s: &str) -> Result<Gender, Error> {
    let normalized_string = s.trim().to_lowercase();
    match normalized_string.as_str() {
        "male" => Ok(Gender::Male),
        "female" => Ok(Gender::Female),
        "nonbinary" | "non binary" | "non-binary" | "none" => Ok(Gender::Female),
        _ => Err(Error),
    }
}

fn class_match_string(s: &str) -> Result<Class, Error> {
    let normalized_string = s.trim().to_lowercase();
    match normalized_string.as_str() {
        "artificer" => Ok(Class::Artificer),
        "barbarian" => Ok(Class::Barbarian),
        "bard" => Ok(Class::Bard),
        "cleric" => Ok(Class::Cleric),
        "druid" => Ok(Class::Druid),
        "fighter" => Ok(Class::Fighter),
        "monk" => Ok(Class::Monk(MonkClass {})),
        "paladin" => Ok(Class::Paladin),
        "ranger" => Ok(Class::Ranger),
        "rogue" => Ok(Class::Rogue),
        "sorcerer" => Ok(Class::Sorcerer),
        "warlock" => Ok(Class::Warlock),
        "wizard" => Ok(Class::Wizard),
        _ => Err(Error),
    }
}

fn background_match_string(s: &str) -> Result<Background, Error> {
    let normalized_string = s.trim().to_lowercase();
    match normalized_string.as_str() {
        "acolyte" => Ok(Background::Acolyte),
        "charlatan" => Ok(Background::Charlatan),
        "criminal" => Ok(Background::Criminal),
        "entertainer" => Ok(Background::Entertainer),
        "folkhero" | "folk hero" => Ok(Background::FolkHero),
        "guildartisan" | "guild artisan" => Ok(Background::GuildArtisan),
        "hermit" => Ok(Background::Hermit),
        "noble" => Ok(Background::Noble),
        "outlander" => Ok(Background::Outlander),
        "sailor" => Ok(Background::Sailor),
        "soldier" => Ok(Background::Soldier),
        "urchin" => Ok(Background::Urchin),
        _ => Err(Error),
    }
}

fn choose_value<T>(
    string_one: &str,
    string_two: &str,
    // matcher: fn(&str) -> Result<T, std::fmt::Error>,
) -> T
where
    T: std::fmt::Debug + IntoEnumIterator + FromStr,
    <T as std::str::FromStr>::Err: std::error::Error,
{
    let one_second = time::Duration::from_secs(1);
    let mut rng = rand::thread_rng();
    loop {
        pretty_print(string_one, BLUE, true);
        pretty_print("Please choose from the following: ", BLUE, false);
        pretty_print(string_two, PURPLE, true);
        pretty_print("(press ENTER to randomize):", PURPLE, false);
        thread::sleep(one_second);
        let mut input_str = String::new();
        match io::stdin().read_line(&mut input_str) {
            Ok(length) => {
                if length > 1 {
                    // let match_value = matcher(&input_str);
                    let match_value = T::from_str(&input_str.trim());
                    println!("match_value: {:?}", match_value);
                    match match_value {
                        Ok(result) => break result,
                        Err(e) => {
                            println!("ERROR: {:?}", e);
                            pretty_print("UNACCEPTABLE", RED, true);
                            continue;
                        }
                    }
                } else {
                    break T::iter().choose(&mut rng).unwrap();
                }
            }
            _ => {
                pretty_print("UNACCEPTABLE", RED, true);
                continue;
            }
        }
    }
}

fn pretty_print(string: &str, color: Color, newline: bool) {
    let newline = match newline {
        true => "\n",
        false => "",
    };
    execute!(
        io::stdout(),
        SetForegroundColor(color),
        Print(string),
        Print(newline),
        ResetColor
    )
    .expect("ERROR: stdout unavailable");
}

fn choose_and_stringify<T>(slice: T) -> String
where
    T: IntoIterator,
    T::Item: Display,
{
    let mut rng = rand::thread_rng();
    slice.into_iter().choose(&mut rng).unwrap().to_string()
}

fn random_name_from_race_gender(race: &Race, gender: &Gender) -> String {
    match race {
        Race::Dragonborn => match gender {
            Gender::Male => choose_and_stringify(names::DRAGONBORN_MALE),
            Gender::Female | Gender::NonBinary => choose_and_stringify(names::DRAGONBORN_FEMALE),
        },
        Race::Dwarf => match gender {
            Gender::Male => choose_and_stringify(names::DWARF_MALE),
            Gender::Female | Gender::NonBinary => choose_and_stringify(names::DWARF_FEMALE),
        },
        Race::Elf => match gender {
            Gender::Male => choose_and_stringify(names::ELF_MALE),
            Gender::Female | Gender::NonBinary => choose_and_stringify(names::ELF_FEMALE),
        },
        Race::Halfling | Race::Gnome => match gender {
            Gender::Male => choose_and_stringify(names::HALFLING_MALE),
            Gender::Female | Gender::NonBinary => choose_and_stringify(names::HALFLING_FEMALE),
        },
        Race::HalfOrc => match gender {
            Gender::Male => choose_and_stringify(names::HALFORC_MALE),
            Gender::Female | Gender::NonBinary => choose_and_stringify(names::HALFORC_FEMALE),
        },
        Race::Human | Race::HalfElf => match gender {
            Gender::Male => choose_and_stringify(names::HUMAN_MALE),
            Gender::Female | Gender::NonBinary => choose_and_stringify(names::HUMAN_FEMALE),
        },
        Race::Tiefling => match gender {
            Gender::Male => choose_and_stringify(names::TIEFLING_MALE),
            Gender::Female | Gender::NonBinary => choose_and_stringify(names::TIEFLING_FEMALE),
        },
    }
}

fn choose_name(race: &Race, gender: &Gender) -> String {
    pretty_print("\nWhat is your character's name?", BLUE, true);
    pretty_print("(press ENTER to randomize):", PURPLE, false);
    // let name = String::from("Osswalkd");
    let mut name = String::new();
    match io::stdin().read_line(&mut name) {
        Ok(length) => {
            if length > 1 {
                name
            } else {
                random_name_from_race_gender(&race, &gender)
            }
        }
        Err(_) => String::from("bar"),
    }
}

fn choose_level() -> u8 {
    loop {
        pretty_print("\nWhat is your character's level?", BLUE, true);
        pretty_print("(press ENTER if Level '1'):", PURPLE, false);
        // let name = String::from("Osswalkd");
        let mut level = String::new();
        match io::stdin().read_line(&mut level) {
            Ok(length) => {
                if length > 1 {
                    match level.trim().parse::<u8>() {
                        Ok(num) => match num {
                            1..=20 => break num,
                            _ => {
                                pretty_print("Level Must Be Between 1 and 20", RED, true);
                                continue;
                            }
                        },
                        Err(x) => {
                            println!("IT IS *NOT* A NUM: {:?}", x);
                            pretty_print("Please Enter a Number Between 1 and 20", RED, true);
                            continue;
                        }
                    }
                } else {
                    break 1;
                }
            }
            _ => {
                pretty_print("ERROR, please try again", RED, true);
                continue;
            }
        }
    }
}

fn create_new_character() -> PlayObject {
    let one_second = time::Duration::from_secs(1);
    pretty_print(
        "It doesn't appear you've created a character yet. Let's get started.",
        BLUE,
        true,
    );
    thread::sleep(one_second);

    let race = Race::choose();
    println!("RACE: {:?}", &race);

    let gender = Gender::choose();
    println!("GENDER: {:?}", &gender);

    let name = choose_name(&race, &gender);
    println!("NAME: {}", &name);

    let class = Class::choose();
    println!("CLASS: {:?}", &class);

    let background = Background::choose();
    println!("BACKGROUND: {:?}", &background);

    let level: u8 = choose_level();
    println!("LEVEL: {}", &level);

    // pretty_print("\nWhat is your character's sub-class?", BLUE, true);
    // pretty_print("(press ENTER to randomize):", PURPLE, true);
    // thread::sleep(one_second);
    //TODO: add subclass to PlayObject

    // pretty_print("\nDo you want to enter your character's stats?", BLUE, true);
    // pretty_print("(press ENTER to default to 'NO'):", PURPLE, true);
    // thread::sleep(one_second);

    let stats = [
        Stat::Str(10),
        Stat::Dex(18),
        Stat::Con(14),
        Stat::Int(8),
        Stat::Wis(16),
        Stat::Chr(8),
    ];

    let status = Status::new(&stats);

    PlayObject {
        character: Character {
            name,
            level,
            background,
            race,
            class,
            stats,
            status,
            gender,
        },
        created_at: Utc::now(),
        updated_at: Some(Utc::now()),
        last_played_at: Utc::now(),
    }
}

fn main() -> Result<(), serde_yaml::Error> {
    // let stats = [
    //     Stat::Str(10),
    //     Stat::Dex(18),
    //     Stat::Con(14),
    //     Stat::Int(8),
    //     Stat::Wis(16),
    //     Stat::Chr(8),
    // ];
    // let stats_clone = [
    //     Stat::Str(10),
    //     Stat::Dex(18),
    //     Stat::Con(14),
    //     Stat::Int(8),
    //     Stat::Wis(16),
    //     Stat::Chr(8),
    // ];
    // let play_object = PlayObject {
    //     character: Character {
    //         name: String::from("Osswald"),
    //         level: 4,
    //         background: Background::Urchin,
    //         race: Race::Human,
    //         class: Class::Monk(MonkClass {}),
    //         stats: stats,
    //         status: Status::new(&stats_clone),
    //     },
    //     created_at: Utc::now(),
    //     updated_at: Some(Utc::now()),
    //     last_played_at: Utc::now(),
    // };

    // let data = fs::read_to_string("./test.yaml").expect("Unable to read file");
    let data = fs::read_to_string("./bad_test.yaml").expect("Unable to read file");
    println!("{}", data);

    // let play_object: PlayObject =
    //     serde_yaml::from_str(&data).expect("Character YAML not properly configured");
    let play_object: PlayObject = match serde_yaml::from_str(&data) {
        Ok(play_object) => play_object,
        Err(err) => {
            println!("playobject error: {:?}", err);
            create_new_character()
        }
    };

    // let s = serde_yaml::to_string(&play_object)?;
    // println!("{:?}", s);
    // fs::write("./output.yaml", &s).expect("Unable to write file");

    play_object.character.display();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_character_yaml() -> Result<(), String> {
        match fs::read_to_string("./test_yaml/input.yaml") {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("Unable to read character YAML")),
        }
    }

    // #[test]
    // fn can_deserialize_character_yaml_as_struct() -> Result<(), serde_yaml::Error> {
    //     let data =
    //         fs::read_to_string("./test_yaml/input2.yaml").expect("Unable to read input file");
    //     let play_object: Result<PlayObject, serde_yaml::Error> = serde_yaml::from_str(&data);
    //     match play_object {
    //         Ok(_) => Ok(()),
    //         Err(e) => Err(e),
    //     }
    // }
    // #[test]
    // fn first_test() {
    //     let data = fs::read_to_string("./test_yaml/input.yaml").expect("Unable to read input file");
    //     let play_object: PlayObject =
    //         serde_yaml::from_str(&data).expect("Character YAML not properly configured");
    //     let s = serde_yaml::to_string(&play_object).expect("Could not serialize PlayObject data");
    //     let expected_output =
    //         fs::read_to_string("./test_yaml/output.yaml").expect("Unable to read output file");
    //     assert_eq!(s, expected_output);
    //     // assert_eq!();
    // }

    // #[test]
    // fn add_two_with_result() -> Result<(), String> {
    //     if add_two(2) == 4 {
    //         Ok(())
    //     } else {
    //         Err(String::from("I guess 2 + 2 doesn't equal 4"))
    //     }
    // }

    // #[test]
    // #[should_panic]
    // fn this_panics() {
    //     panic!("Yikes!")
    // }
}
