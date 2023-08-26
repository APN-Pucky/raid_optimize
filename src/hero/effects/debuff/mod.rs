use enum_map::Enum;

pub mod attribute;
pub mod dot;

#[derive(Debug,Enum, PartialEq, Eq ,strum_macros::Display,Copy,Clone,Deserialize, )]
pub enum Debuff {
    WetI,
    WetII,
    ColdI,
    ColdII,
    ColdIII,
    Bleed,
    SpeedDownI,
    SpeedDownII,
    EffectResistanceDownI,
    EffectResistanceDownII,

}
