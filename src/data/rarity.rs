#[derive(Debug, PartialEq,strum_macros::Display,strum_macros::EnumIter, Deserialize, Serialize, Clone,Copy )]
pub enum Rarity {
    Common, // White
    Rare, // Green
    Elite, // Blue
    Epic, // Purple
    Legendary, // Gold
    Mythic, // Red
}