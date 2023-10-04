#[derive(Debug, PartialEq,strum_macros::Display, strum_macros::EnumIter, Deserialize, Serialize, Clone,Copy )]
pub enum Class {
    Attack,
    Support,
    Defense,
}