#[derive(Debug, PartialEq,strum_macros::Display, Deserialize, Clone )]
pub enum Rarity {
    Common, // White
    Rare, // Green
    Elite, // Blue
    Epic, // Purple
    Legendary, // Gold
}