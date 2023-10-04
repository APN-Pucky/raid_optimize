
use enum_map::{Enum, EnumMap};
use strum_macros::EnumIter;

//#[derive(Debug, PartialEq,strum_macros::Display, Deserialize, Serialize, Clone )]
#[derive(Debug,Enum, PartialEq, Eq, strum_macros::Display,strum_macros::EnumIter,Deserialize, Serialize,Copy,Clone)]
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
    HiddenWave,
    // In progress
    // TODO
}