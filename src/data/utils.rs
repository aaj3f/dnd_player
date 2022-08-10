use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use rand::prelude::*;
use std::fmt::Display;
use std::string::ToString;
use std::{
    io::{self},
    str::FromStr,
};
use strum::IntoEnumIterator;

pub const _PURPLE: Color = Color::Rgb {
    r: 183,
    g: 117,
    b: 214,
};

pub const BLUE: Color = Color::Rgb {
    r: 19,
    g: 198,
    b: 255,
};

pub const RED: Color = Color::Rgb {
    r: 244,
    g: 67,
    b: 54,
};

pub fn pretty_print(string: &str, color: Color, newline: bool) {
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

pub fn choose_yes_or_no(character_name: &str) -> bool {
    let choice = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(&format!(
            "Would you like to continue with your previous character, {}?",
            character_name
        ))
        .default(true)
        .show_default(false)
        .wait_for_newline(true)
        .interact()
        .unwrap();
    if choice {
        println!("Great, let's play as {}.", character_name);
    } else {
        println!("Ok! Let's get started with a new character.");
    };
    choice

    // loop {
    //     let selections = &["Yes", "No"];

    //     let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
    //         .with_prompt(string_choice)
    //         .default(0)
    //         .items(&selections[..])
    //         .interact();

    //     match selection {
    //         Ok(index) => match index {
    //             0 => break true,
    //             1 => break false,
    //             _ => continue,
    //         },
    //         Err(_) => continue,
    //     }
    // }
}

pub trait StringJoin<T> {
    fn join_string() -> String;
    fn collect_string() -> Vec<String>;
}

impl<T> StringJoin<T> for T
where
    T: Display + IntoEnumIterator,
{
    fn collect_string() -> Vec<String> {
        T::iter().map(|x| x.to_string()).collect::<Vec<String>>()
    }

    fn join_string() -> String {
        T::collect_string().join(", ")
    }
}

pub trait Choosable<T> {
    fn choose() -> T;
}

pub fn choose_value<T>(string_one: &str, selections: &Vec<String>) -> T
where
    T: std::fmt::Debug + IntoEnumIterator + FromStr + Display,
    <T as std::str::FromStr>::Err: std::error::Error,
{
    let mut rng = rand::thread_rng();
    let mut fuzzy_selections = selections.clone();
    fuzzy_selections.insert(0, String::from("Random"));

    pretty_print(string_one, BLUE, true);
    let selection_result = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Please choose from the following:")
        .default(0)
        .max_length(5)
        .items(&fuzzy_selections[..])
        .interact();

    let result = match selection_result {
        Ok(u) => match u {
            0 => T::iter().choose(&mut rng).unwrap(),
            _ => match &fuzzy_selections[..].get(u) {
                Some(selection_string) => T::from_str(selection_string).unwrap(),
                None => T::iter().choose(&mut rng).unwrap(),
            },
        },
        Err(_) => T::iter().choose(&mut rng).unwrap(),
    };
    pretty_print(&format!("\nYour choice: {}\n", result), BLUE, true);
    result
}
