use enum_map::Enum;

pub mod attribute;

#[derive(Debug,Enum, PartialEq, Eq ,strum_macros::Display,Copy,Clone,Deserialize, )]
pub enum Buff {
    Stealth,
    Heal,
    BlockDebuf,
}