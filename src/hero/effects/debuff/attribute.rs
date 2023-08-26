use enum_map::Enum;

#[derive(Debug,Enum, PartialEq, Eq ,strum_macros::Display,Copy,Clone,Deserialize, )]
pub enum AttributeDebuff {
    WetI,
    WetII,
    ColdI,
    ColdII,
    ColdIII,
    SpeedDownI,
    SpeedDownII,
    EffectResistanceDownI,
    EffectResistanceDownII,
}