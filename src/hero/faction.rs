
use enum_map::{Enum, EnumMap};


//#[derive(Debug, PartialEq,strum_macros::Display, Deserialize, Clone )]
#[derive(Debug,Enum, PartialEq, Eq, strum_macros::Display,Deserialize,Copy,Clone)]
pub enum Faction {
    Foresters,
    DoomLegion,
    EternalSect,
    HiddenWave,
    WizardsEye,
    DragonTribe,
    HolyLightParliament,
    SunsetSages
}