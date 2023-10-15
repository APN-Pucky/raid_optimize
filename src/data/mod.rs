use quick_xml::de::from_str;

pub mod class;
pub mod effect;
pub mod faction;
pub mod hero;
pub mod heroes;
pub mod instance;
pub mod mark;
pub mod rarity;
pub mod skill;
pub mod subskill;

use self::heroes::Heroes;

pub fn load_heroes(heroes_xml: String) -> Heroes {
    let file_string = std::fs::read_to_string(heroes_xml).unwrap();
    let heroes: Heroes = from_str(&file_string).unwrap();
    heroes
}
