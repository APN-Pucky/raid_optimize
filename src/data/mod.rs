
use quick_xml::de::from_str;


pub mod instance;
pub mod skill;
pub mod effect;
pub mod subskill;
pub mod class;
pub mod faction;
pub mod rarity;
pub mod mark;
pub mod hero;
pub mod heroes;


use self::{ heroes::Heroes};


pub fn load_heroes(heroes_xml :String ) -> Heroes {
    let file_string = std::fs::read_to_string(heroes_xml).unwrap();
    let heroes : Heroes = from_str(&file_string).unwrap();
    heroes
}