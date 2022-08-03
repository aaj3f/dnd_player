mod data;

use chrono::prelude::*;
use data::background::*;
use data::character::*;
use data::classes::*;
use data::gender::*;
use data::names;
use data::races::*;
use data::stats::*;
use data::utils::*;
use rand::prelude::*;
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml;
use std::fmt::Display;
use std::{
    fs,
    io::{self},
    thread, time,
};

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
            Gender::Female | Gender::None => choose_and_stringify(names::DRAGONBORN_FEMALE),
        },
        Race::Dwarf(_) => match gender {
            Gender::Male => choose_and_stringify(names::DWARF_MALE),
            Gender::Female | Gender::None => choose_and_stringify(names::DWARF_FEMALE),
        },
        Race::Elf(_) => match gender {
            Gender::Male => choose_and_stringify(names::ELF_MALE),
            Gender::Female | Gender::None => choose_and_stringify(names::ELF_FEMALE),
        },
        Race::Halfling(_) | Race::Gnome(_) => match gender {
            Gender::Male => choose_and_stringify(names::HALFLING_MALE),
            Gender::Female | Gender::None => choose_and_stringify(names::HALFLING_FEMALE),
        },
        Race::HalfOrc => match gender {
            Gender::Male => choose_and_stringify(names::HALFORC_MALE),
            Gender::Female | Gender::None => choose_and_stringify(names::HALFORC_FEMALE),
        },
        Race::Human(_) | Race::HalfElf => match gender {
            Gender::Male => choose_and_stringify(names::HUMAN_MALE),
            Gender::Female | Gender::None => choose_and_stringify(names::HUMAN_FEMALE),
        },
        Race::Tiefling => match gender {
            Gender::Male => choose_and_stringify(names::TIEFLING_MALE),
            Gender::Female | Gender::None => choose_and_stringify(names::TIEFLING_FEMALE),
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
                name.trim().to_owned()
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

fn choose_average_dice() -> bool {
    pretty_print("\nTo calculate your player's HP, would you like to\ntake the average of your hit dice [Y] or roll for each new level [N]?", BLUE, true);
    loop {
        pretty_print("[Y/N]: ", PURPLE, false);
        // let name = String::from("Osswalkd");
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
    }
}

fn match_stat(stat: &Stat, new_value: u8) -> Stat {
    match stat {
        Stat::Str(_) => Stat::Str(new_value),
        Stat::Dex(_) => Stat::Dex(new_value),
        Stat::Con(_) => Stat::Con(new_value),
        Stat::Int(_) => Stat::Int(new_value),
        Stat::Wis(_) => Stat::Wis(new_value),
        Stat::Chr(_) => Stat::Chr(new_value),
    }
}

fn choose_stats() -> [Stat; 6] {
    pretty_print("You will need to choose the following stats...", BLUE, true);
    pretty_print(&Stat::list(), PURPLE, true);
    let mut result = [
        Stat::Str(0),
        Stat::Dex(0),
        Stat::Con(0),
        Stat::Int(0),
        Stat::Wis(0),
        Stat::Chr(0),
    ];
    for stat in &mut result {
        loop {
            pretty_print(
                &format!("Enter a value for {}: ", stat.show_name()),
                BLUE,
                false,
            );
            let mut stat_value = String::from("");
            match io::stdin().read_line(&mut stat_value) {
                Ok(_) => match stat_value.trim().parse::<u8>().unwrap_or(21) {
                    x if (1..=20).contains(&x) => {
                        *stat = match_stat(&stat, x);
                        break;
                    }
                    _ => continue,
                },
                Err(_) => continue,
            }
        }
    }
    result
}

fn create_new_character() -> PlayObject {
    let one_second = time::Duration::from_secs(1);
    pretty_print("Let's get started.", BLUE, true);
    thread::sleep(one_second);

    let race = Race::choose();
    println!("RACE: {:?}", &race);

    let race: Race = match race {
        Race::Dwarf(_) => race.choose_subrace(),
        Race::Elf(_) => race.choose_subrace(),
        Race::Halfling(_) => race.choose_subrace(),
        Race::Human(_) => race.choose_subrace(),
        Race::Gnome(_) => race.choose_subrace(),
        _ => race,
    };
    // let race: Race = match race {
    //     Race::Dwarf(_) => choose_subrace(Dwarf::iter().collect()),
    //     Race::Elf(_) => choose_subrace(Elf::iter().collect()),
    //     Race::Halfling(_) => choose_subrace(Halfling::iter().collect()),
    //     Race::Human(_) => choose_subrace(Human::iter().collect()),
    //     Race::Gnome(_) => choose_subrace(Gnome::iter().collect()),
    //     _ => race,
    // };

    let gender = Gender::choose();
    println!("GENDER: {:?}", &gender);

    let name = choose_name(&race, &gender);
    println!("NAME: {}", &name);

    let mut class = Class::choose();
    println!("CLASS: {:?}", &class);

    let background = Background::choose();
    println!("BACKGROUND: {:?}", &background);

    let level: u8 = choose_level();
    println!("LEVEL: {}", &level);

    if level > 2 {
        class = class.choose_subclass();
        thread::sleep(one_second);
        //TODO: add subclass to PlayObject
    }

    let stats: [Stat; 6] = choose_stats();
    println!("STATS: {:?}", &stats);

    // pretty_print("\nDo you want to enter your character's stats?", BLUE, true);
    // pretty_print("(press ENTER to default to 'NO'):", PURPLE, true);
    // thread::sleep(one_second);

    /*
    Have you already determined your character's stats, or would you like to use a stat calculator?
    - ALREADY KNOW
        Do you want to verify that you've correctly allocated
    - CALCULATOR
    */

    // let stats = [
    //     Stat::Str(10),
    //     Stat::Dex(18),
    //     Stat::Con(14),
    //     Stat::Int(8),
    //     Stat::Wis(16),
    //     Stat::Chr(8),
    // ];
    let use_average_dice = choose_average_dice();
    let status = Status::new(&stats, &race, &class, &level, use_average_dice);

    let character = Character {
        name,
        level,
        background,
        race,
        class,
        stats,
        status,
        gender,
    };

    PlayObject {
        character,
        created_at: Utc::now(),
        updated_at: Some(Utc::now()),
        last_played_at: Utc::now(),
    }
}

fn load_character_or_new(play_object: PlayObject) -> PlayObject {
    let string_choice = format!(
            "Would you like to continue with your previous character, {}?\n(Selecting no [N] will have you create a new character) [Y/N]: ",
            play_object.character.name
        );

    play_object.character.display(true);

    if choose_yes_or_no(&string_choice) {
        play_object
    } else {
        create_new_character()
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

    let data = fs::read_to_string("./test.yaml").expect("Unable to read file");
    // let data = fs::read_to_string("./bad_test.yaml").expect("Unable to read file");
    println!("{}", data);

    // let play_object: PlayObject =
    //     serde_yaml::from_str(&data).expect("Character YAML not properly configured");
    let play_object: PlayObject = match serde_yaml::from_str(&data) {
        Ok(play_object) => load_character_or_new(play_object),
        Err(_err) => {
            // println!("playobject error: {:?}", err);
            pretty_print(
                "It doesn't appear you've created a character yet",
                BLUE,
                true,
            );
            create_new_character()
        }
    };

    let s = serde_yaml::to_string(&play_object)?;
    println!("{:?}", s);
    fs::write("./output.yaml", &s).expect("Unable to write file");

    play_object.character.display(true);
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
