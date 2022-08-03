use rand::{thread_rng, Rng};
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
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Status {
    armor_class: i8,
    conditions: Condition,
    blessed: bool,
    initiative: i8,
    hit_dice: Dice,
    current_hp: i8,
    maximum_hp: i8,
    speed: u16,
}

fn calculate_ac(stats: &[Stat], class: &Class) -> i8 {
    let additional_ac = match class {
        Class::Barbarian(_) => stats[2].get_modifier(),
        Class::Monk(_) => stats[4].get_modifier(),
        _ => 0,
    };
    10 + stats[1].get_modifier() + additional_ac
}

fn calculate_speed(race: &Race) -> u16 {
    match race {
        Race::Dwarf(_) | Race::Gnome(_) | Race::Halfling(_) => 25,
        Race::Dragonborn | Race::HalfElf | Race::HalfOrc | Race::Human(_) | Race::Tiefling => 30,
        Race::Elf(v) => match v {
            super::races::Elf::WoodElf => 35,
            _ => 30,
        },
    }
}

impl Status {
    pub fn new(
        stats: &[Stat],
        race: &Race,
        class: &Class,
        level: &u8,
        use_average_dice: bool,
    ) -> Status {
        let hit_dice = match class {
            Class::Artificer(_) => Dice::D8,
            Class::Barbarian(_) => Dice::D12,
            Class::Bard(_) => Dice::D8,
            Class::Cleric(_) => Dice::D8,
            Class::Druid(_) => Dice::D8,
            Class::Fighter(_) => Dice::D10,
            Class::Monk(_) => Dice::D8,
            Class::Paladin(_) => Dice::D10,
            Class::Ranger(_) => Dice::D10,
            Class::Rogue(_) => Dice::D8,
            Class::Sorcerer(_) => Dice::D6,
            Class::Warlock(_) => Dice::D8,
            Class::Wizard(_) => Dice::D6,
        };
        let status = Status {
            armor_class: calculate_ac(stats, class),
            conditions: Condition::None,
            blessed: false,
            initiative: stats[1].get_modifier(),
            hit_dice,
            current_hp: 10,
            maximum_hp: 10,
            speed: calculate_speed(race),
        };
        status.calculate_hp(level, stats[2].get_modifier(), use_average_dice)
    }

    pub fn calculate_hp(mut self, level: &u8, con_modifier: i8, use_average_dice: bool) -> Self {
        let hit_dice = &self.hit_dice;

        let hp_first_level: i8 = con_modifier
            + match hit_dice {
                Dice::D6 => 6,
                Dice::D8 => 8,
                Dice::D10 => 10,
                Dice::D12 => 12,
                _ => panic!("Somehow you don't have a d6, d8, d10, or d12 as your Hit Dice"),
            };

        let maximum_hp: i8 = if use_average_dice {
            let hp_per_level = match hit_dice {
                Dice::D6 => 4,
                Dice::D8 => 5,
                Dice::D10 => 6,
                Dice::D12 => 7,
                _ => panic!("Somehow you don't have a d6, d8, d10, or d12 as your Hit Dice"),
            };
            println!(
                "HP FIRST LEVEL: {}\nCON MODIFIER VALUE: {}\nHP PER LEVEL: {}\nLEVEL: {}\n",
                hp_first_level, con_modifier, hp_per_level, level
            );
            hp_first_level + ((con_modifier + hp_per_level) * (*level - 1) as i8)
        } else {
            let mut hp_to_add = 0;
            let mut rng = thread_rng();
            for _ in 1..*level {
                let inc = match hit_dice {
                    Dice::D6 => rng.gen_range(1..=6),
                    Dice::D8 => rng.gen_range(1..=8),
                    Dice::D10 => rng.gen_range(1..=10),
                    Dice::D12 => rng.gen_range(1..=12),
                    _ => panic!("Somehow you don't have a d6, d8, d10, or d12 as your Hit Dice"),
                };
                println!(
                    "HP TO ADD: {}\nINC: {}\nCON MOD VALUE: {}\nHP TO ADD (AFTER): {}\n",
                    hp_to_add,
                    inc,
                    con_modifier,
                    inc + con_modifier
                );
                hp_to_add += inc + con_modifier
            }
            hp_first_level + hp_to_add
        };

        self.maximum_hp = maximum_hp;
        self.current_hp = maximum_hp;
        self
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

            let subclass = match &self.class {
                Class::Artificer(v) => v.to_string(),
                Class::Barbarian(v) => v.to_string(),
                Class::Bard(v) => v.to_string(),
                Class::Cleric(v) => v.to_string(),
                Class::Druid(v) => v.to_string(),
                Class::Fighter(v) => v.to_string(),
                Class::Monk(v) => v.to_string(),
                Class::Paladin(v) => v.to_string(),
                Class::Ranger(v) => v.to_string(),
                Class::Rogue(v) => v.to_string(),
                Class::Sorcerer(v) => v.to_string(),
                Class::Warlock(v) => v.to_string(),
                Class::Wizard(v) => v.to_string(),
            };

            table.add_row(Row::new(vec![
                TableCell::new_with_alignment("Class", 1, Alignment::Center),
                TableCell::new_with_alignment(
                    format!("{} ({})", self.class, subclass),
                    11,
                    Alignment::Left,
                ),
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
