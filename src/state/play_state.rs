/*
We should be able to...

- Instantiate play state with a play object
- call a turn() on play_state
- return available actions
- return available bonus actions
- return available reactions
*/

use std::{cmp::Ordering, str::FromStr};

use dialoguer::{theme::ColorfulTheme, Select};
use strum_macros::{Display, EnumIter, EnumString};

use crate::data::{
    spell::Spell,
    utils::{pretty_print, StringJoin, BLUE, LEDGER_URL},
};

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

    pub async fn take_turn(&mut self) {
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
            TurnOption::Action => self.get_actions().await,
            TurnOption::BonusAction => self.get_bonus_actions(),
            TurnOption::Quit => self.quit(),
        }

        self.play_object.character.display(false)
    }

    pub fn get_movement(&self) {}

    pub async fn get_actions(&self) {
        let url = format!("{}/query", LEDGER_URL);
        let class = &self.play_object.character.class.to_string();
        let current_spell_slots = &self.play_object.character.status.current_spell_slots;
        let spell_level = &current_spell_slots[current_spell_slots.len() - 1].level;
        let spell_level_string = &format!("#(< ?level {})", spell_level);
        let json_message = &serde_json::json!({
          "select": {
              "?spell": ["name", "level"]
          },
          "where": [
              ["?spell", "spell/level", spell_level_string],
              ["?spell", "spell/available_to", ["class/name", class]]
              ],
          "opts": {
             "limit": 999999
          }
        });
        // println!("{:#?}", json_message);
        let mut spells: Vec<Spell> = reqwest::Client::new()
            .post(url)
            .json(json_message)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        spells.sort_by(|a, b| match a.level.cmp(&b.level) {
            Ordering::Equal => a.name.cmp(&b.name),
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        });

        let _selection_result = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which spell would you like to cast?")
            .default(0)
            .max_length(10)
            .items(&spells)
            .interact()
            .unwrap();
    }

    pub fn get_bonus_actions(&self) {}

    pub fn quit(&mut self) {
        self.active = false;
    }

    // pub fn get_reactions(&self) {}
}
