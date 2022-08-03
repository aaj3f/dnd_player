use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use rand::prelude::*;
use std::fmt::Display;
use std::string::ToString;
use std::{
    io::{self},
    str::FromStr,
    thread, time,
};
use strum::IntoEnumIterator;

pub const PURPLE: Color = Color::Rgb {
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

pub fn choose_yes_or_no(string_choice: &str) -> bool {
    loop {
        pretty_print(string_choice, BLUE, false);

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

pub trait Choosable<T> {
    fn choose() -> T;
}

pub fn choose_value<T>(string_one: &str, string_two: &str) -> T
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
