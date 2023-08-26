#[derive(Debug, PartialEq,strum_macros::Display, Deserialize, Clone )]
pub enum Class {
    Attack,
    Support,
    Defense,
}