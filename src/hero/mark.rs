#[derive(Debug, PartialEq,strum_macros::Display, Deserialize, Clone )]
pub enum Mark {
    Blue,
    Red,
    Green,
    Force
}