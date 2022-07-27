use serde::{Deserialize, Serialize};
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};

use super::{background::Background, classes::Class, gender::Gender, races::Race, stats::Stat};

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
pub struct Status {
    armor_class: u8,
    conditions: Condition,
    blessed: bool,
    initiative: u8,
    hit_dice: Dice,
    current_hp: u8,
    maximum_hp: u8,
    speed: u16,
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
pub struct Character {
    pub name: String,
    pub level: u8,
    pub background: Background,
    pub race: Race,
    pub class: Class,
    pub stats: [Stat; 6],
    pub status: Status,
    pub gender: Gender,
}

impl Character {
    pub fn display(&self, verbose: bool) {
        let mut table = Table::new();
        // table.max_column_width = 40;

        table.style = TableStyle::extended();

        if verbose {
            table.add_row(Row::new(vec![TableCell::new_with_alignment(
                "Character Sheet",
                12,
                Alignment::Center,
            )]));

            table.add_row(Row::new(vec![
                TableCell::new_with_alignment("Name", 1, Alignment::Center),
                TableCell::new_with_alignment(format!("{}", self.name), 11, Alignment::Left),
            ]));

            table.add_row(Row::new(vec![
                TableCell::new_with_alignment("Gender", 1, Alignment::Center),
                TableCell::new_with_alignment(format!("{}", self.gender), 11, Alignment::Left),
            ]));
            table.add_row(Row::new(vec![
                TableCell::new_with_alignment("Race", 1, Alignment::Center),
                TableCell::new_with_alignment(format!("{}", self.race), 11, Alignment::Left),
            ]));

            table.add_row(Row::new(vec![
                TableCell::new_with_alignment("Class", 1, Alignment::Center),
                TableCell::new_with_alignment(format!("{}", self.class), 11, Alignment::Left),
            ]));

            table.add_row(Row::new(vec![
                TableCell::new_with_alignment("Level", 1, Alignment::Center),
                TableCell::new_with_alignment(format!("{}", self.level), 11, Alignment::Left),
            ]));

            table.add_row(Row::new(vec![
                TableCell::new_with_alignment("Background", 1, Alignment::Center),
                TableCell::new_with_alignment(format!("{}", self.background), 11, Alignment::Left),
            ]));

            table.add_row(Row::new(vec![TableCell::new_with_col_span("", 12)]));
        }

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

        // let stats_array: Vec<String> = self.stats.iter().map(|x| x.display()).collect();

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
            TableCell::new_with_alignment(self.get_str().display(), 1, Alignment::Center),
            TableCell::new_with_alignment(self.get_dex().display(), 1, Alignment::Center),
            TableCell::new_with_alignment(self.get_con().display(), 1, Alignment::Center),
            TableCell::new_with_alignment(self.get_int().display(), 1, Alignment::Center),
            TableCell::new_with_alignment(self.get_wis().display(), 1, Alignment::Center),
            TableCell::new_with_alignment(self.get_chr().display(), 1, Alignment::Center),
        ]));

        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            format!("Status Conditions: None"),
            12,
            Alignment::Left,
        )]));

        println!("{}", table.render());
    }
    pub fn get_str(&self) -> &Stat {
        &self.stats[0]
    }
    pub fn get_dex(&self) -> &Stat {
        &self.stats[1]
    }
    pub fn get_con(&self) -> &Stat {
        &self.stats[2]
    }
    pub fn get_int(&self) -> &Stat {
        &self.stats[3]
    }
    pub fn get_wis(&self) -> &Stat {
        &self.stats[4]
    }
    pub fn get_chr(&self) -> &Stat {
        &self.stats[5]
    }
}
