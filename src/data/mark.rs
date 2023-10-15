use enum_map::Enum;
use strum_macros::EnumIter;

#[derive(
    Debug, Enum, PartialEq, strum_macros::Display, EnumIter, Deserialize, Serialize, Clone, Copy,
)]
pub enum Mark {
    Blue,
    Red,
    Green,
    Force,
}
