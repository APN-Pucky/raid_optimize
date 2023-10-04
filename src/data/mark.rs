
use strum_macros::EnumIter;
use enum_map::{Enum, EnumMap};
use std::fmt;

#[derive(Debug, Enum,PartialEq, strum_macros::Display, EnumIter, Deserialize, Serialize, Clone,Copy )]
pub enum Mark {
    Blue,
    Red,
    Green,
    Force
}
