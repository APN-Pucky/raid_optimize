
use enum_map::{Enum, EnumMap};

#[derive(Debug, Enum,PartialEq,strum_macros::Display, Deserialize, Clone,Copy )]
pub enum Mark {
    Blue,
    Red,
    Green,
    Force
}
