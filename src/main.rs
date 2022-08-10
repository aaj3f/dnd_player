mod data;
mod state;

use chrono::prelude::*;
use data::background::*;
use data::character::*;
use data::classes::*;
use data::gender::*;
use data::names;
use data::races::*;
use data::stats::*;
use data::utils::*;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;
use dialoguer::Input;
use rand::prelude::*;
use serde_yaml;
use state::play_object::PlayObject;
use std::fmt::Display;
use std::{
    fs,
    io::{self},
    thread, time,
};

use crate::state::play_state::PlayState;

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
    let name_result: Result<String, io::Error> = Input::with_theme(&ColorfulTheme::default())
        .allow_empty(true)
        .show_default(false)
        .with_prompt("What is your character's name? (press ENTER to randomize)")
        .interact_text();
    let result = match name_result {
        Ok(name) => {
            let name = name.trim();
            if name.chars().count() > 0 {
                name.to_owned()
            } else {
                random_name_from_race_gender(&race, &gender)
            }
        }
        Err(_) => random_name_from_race_gender(&race, &gender),
    };

    pretty_print(&format!("\nYour choice: {}\n", result), BLUE, true);
    result
}

fn is_valid_level(input: &String) -> bool {
    match input.parse::<u8>() {
        Ok(v) => match v {
            1..=20 => true,
            _ => false,
        },
        Err(_) => false,
    }
}

fn choose_level() -> u8 {
    let level: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your character's level?")
        .default(String::from("1"))
        .validate_with(|input: &String| -> Result<(), &str> {
            if is_valid_level(&input) {
                Ok(())
            } else {
                Err("That is not a valid level")
            }
        })
        .interact_text()
        .unwrap();
    pretty_print(&format!("\nYour choice: {:?}\n", level), BLUE, true);
    level.parse::<u8>().unwrap()
    // loop {
    //     pretty_print("\nWhat is your character's level?", BLUE, true);
    //     pretty_print("(press ENTER if Level '1'):", PURPLE, false);
    //     // let name = String::from("Osswalkd");
    //     let mut level = String::new();
    //     match io::stdin().read_line(&mut level) {
    //         Ok(length) => {
    //             if length > 1 {
    //                 match level.trim().parse::<u8>() {
    //                     Ok(num) => match num {
    //                         1..=20 => break num,
    //                         _ => {
    //                             pretty_print("Level Must Be Between 1 and 20", RED, true);
    //                             continue;
    //                         }
    //                     },
    //                     Err(x) => {
    //                         println!("IT IS *NOT* A NUM: {:?}", x);
    //                         pretty_print("Please Enter a Number Between 1 and 20", RED, true);
    //                         continue;
    //                     }
    //                 }
    //             } else {
    //                 break 1;
    //             }
    //         }
    //         _ => {
    //             pretty_print("ERROR, please try again", RED, true);
    //             continue;
    //         }
    //     }
    // }
}

fn choose_average_dice() -> bool {
    let choice = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to calculate your player's HP with the average of your hit dice? ('NO' will roll for each new level)")
        .default(true)
        .show_default(false)
        .wait_for_newline(true)
        .interact()
        .unwrap();
    if choice {
        println!("Great, we'll use your hit dice average!");
    } else {
        println!("Brave choice! Let's roll your HP for each level");
    };
    choice
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
    pretty_print(
        &format!("You will need to provide values for {}...", &Stat::list()),
        BLUE,
        true,
    );

    let mut result = [
        Stat::Str(0),
        Stat::Dex(0),
        Stat::Con(0),
        Stat::Int(0),
        Stat::Wis(0),
        Stat::Chr(0),
    ];
    for stat in &mut result {
        let prompt = format!("Enter a value for {}:", stat.show_name());
        let stat_input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(&prompt)
            .default(String::from("10"))
            .show_default(true)
            .validate_with(|input: &String| -> Result<(), &str> {
                if is_valid_level(&input) {
                    Ok(())
                } else {
                    Err("That is not a valid stat range")
                }
            })
            .interact_text()
            .unwrap();
        let stat_value: u8 = stat_input.parse().unwrap();
        *stat = match_stat(&stat, stat_value);
    }
    println!("\n");
    result
}

fn create_new_character() -> PlayObject {
    let one_second = time::Duration::from_secs(1);
    // pretty_print("Let's get started.", BLUE, true);
    thread::sleep(one_second);

    let race = Race::choose();

    let race: Race = match race {
        Race::Dwarf(_) => race.choose_subrace(),
        Race::Elf(_) => race.choose_subrace(),
        Race::Halfling(_) => race.choose_subrace(),
        Race::Human(_) => race.choose_subrace(),
        Race::Gnome(_) => race.choose_subrace(),
        _ => race,
    };

    let gender = Gender::choose();

    let name = choose_name(&race, &gender);

    let mut class = Class::choose();

    let level: u8 = choose_level();

    if level > 2 {
        class = class.choose_subclass();
    }

    let background = Background::choose();

    let stats: [Stat; 6] = choose_stats();

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
    play_object.character.display(true);

    if choose_yes_or_no(&play_object.character.name) {
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

    let data = match fs::read_to_string("./output.yaml") {
        Ok(s) => s,
        Err(_) => {
            pretty_print("No character data found on file.", RED, true);
            String::from("")
        }
    };
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
    // println!("{:?}", s);
    fs::write("./output.yaml", &s).expect("Unable to write file");

    play_object.character.display(true);

    let mut play_state = PlayState::new(play_object);

    while play_state.active {
        play_state.take_turn()
    }

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
