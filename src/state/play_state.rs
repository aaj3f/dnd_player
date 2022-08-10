/*
We should be able to...

- Instantiate play state with a play object
- call a turn() on play_state
- return available actions
- return available bonus actions
- return available reactions
*/

use std::str::FromStr;

use dialoguer::{theme::ColorfulTheme, Select};
use strum_macros::{Display, EnumIter, EnumString};

use crate::data::utils::{pretty_print, StringJoin, BLUE};

use super::play_object::PlayObject;

#[derive(EnumIter, EnumString, Display)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
enum TurnOption {
    Move,
    Action,
    BonusAction,
    Quit,
}

pub struct PlayState {
    play_object: PlayObject,
    pub active: bool,
}

impl PlayState {
    pub fn new(play_object: PlayObject) -> PlayState {
        PlayState {
            play_object,
            active: true,
        }
    }

    pub fn take_turn(&mut self) {
        let turn_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Your turn begins. What would you like to do?")
            .default(0)
            .items(&TurnOption::collect_string())
            .interact()
            .unwrap();

        let turn_string = &TurnOption::collect_string()[turn_selection];

        pretty_print(&format!("\nYou chose to {}.\n", turn_string), BLUE, true);

        let turn = TurnOption::from_str(turn_string).unwrap();

        match turn {
            TurnOption::Move => self.get_movement(),
            TurnOption::Action => self.get_actions(),
            TurnOption::BonusAction => self.get_bonus_actions(),
            TurnOption::Quit => self.quit(),
        }

        self.play_object.character.display(false)
    }

    pub fn get_movement(&self) {}

    pub fn get_actions(&self) {}

    pub fn get_bonus_actions(&self) {}

    pub fn quit(&mut self) {
        self.active = false;
    }

    // pub fn get_reactions(&self) {}
}
