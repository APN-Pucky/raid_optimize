#[derive(Debug, PartialEq,strum_macros::Display, Deserialize, Clone )]
pub enum Faction {
    EternalSect,
    Foresters,
    DoomLegion,
    HiddenWave,
    WizardsEye,
    DragonTribe,
    HolyLightParliament,
    SunsetSages
}