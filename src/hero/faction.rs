
use enum_map::{Enum, EnumMap};


//#[derive(Debug, PartialEq,strum_macros::Display, Deserialize, Clone )]
#[derive(Debug,Enum, PartialEq, Eq, strum_macros::Display,Deserialize,Copy,Clone)]
pub enum Faction {
    // Stabilized
    // Tested
    // Prototyped
    Foresters,
    DoomLegion,
    WizardsEye,
    SunsetSages,
    EternalSect,
    DragonTribe,
    HolyLightParliament,
    NamelessBrotherhood,
    TheForgotten,
    SwordHarborGuards,
    // In progress
    HiddenWave,
    // TODO
}