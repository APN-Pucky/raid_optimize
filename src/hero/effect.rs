use enum_map::Enum;

#[derive(Debug,Enum, PartialEq, Eq ,strum_macros::Display,Copy,Clone,Deserialize, )]
pub enum Effect {
    WetI,
    WetII,
    ColdI,
    ColdII,
    ColdIII,
    Bleed,
    HPBurning,
    Suffocated,
    BlockDebuf,
    EffectResistanceDownII,
    RippleII,
    AttackUpII,
    TenacityUpII,
    Heal,
    SpeedDownII,
    SpeedUpI,
    None,
}

pub fn is_dot(effect : Effect) -> bool {
    match effect {
        Effect::WetI => false,
        Effect::WetII => false,
        Effect::ColdI => false,
        Effect::ColdII => false,
        Effect::ColdIII => false,
        Effect::Bleed => true,
        Effect::HPBurning => true,
        Effect::Suffocated => false,
        Effect::BlockDebuf => false,
        Effect::EffectResistanceDownII => false,
        Effect::RippleII => false,
        Effect::AttackUpII => false,
        Effect::TenacityUpII => false,
        Effect::Heal => false,
        Effect::SpeedDownII => false,
        Effect::SpeedUpI => false,
        Effect::None => false,
    }
}