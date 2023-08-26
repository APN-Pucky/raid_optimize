use enum_map::Enum;

use super::effects::Effect;

#[derive(Debug,Enum, PartialEq, Eq, strum_macros::Display,Copy,Clone)]
pub enum Stat {
    Attacks,
    Attack,
    CriticalStrikes,
    CriticalDamage,
    TenacityIgnored,
    PiercedDefense,
    DamageDone,
    DamageTaken,
    HealthHealed,
    HealthRestored,
    HealthLost,
    Leeched,
    TurnMeterReduced,
    ShieldBlocked,
    EffectInflicted,
}

pub fn effect_to_stat(e:Effect) -> Stat {
    match e {
        Effect::WetI => Stat::EffectInflicted,
        Effect::WetII => Stat::EffectInflicted,
        Effect::ColdI => Stat::EffectInflicted,
        Effect::ColdII => Stat::EffectInflicted,
        Effect::ColdIII => Stat::EffectInflicted,
        Effect::Bleed => Stat::EffectInflicted,
        Effect::HPBurning => Stat::EffectInflicted,
        Effect::Suffocated => Stat::EffectInflicted,
        Effect::BlockDebuf => Stat::EffectInflicted,
        Effect::EffectResistanceDownII => Stat::EffectInflicted,
        Effect::RippleII => Stat::EffectInflicted,
        Effect::AttackUpII => Stat::EffectInflicted,
        Effect::TenacityUpII => Stat::EffectInflicted,
        Effect::Heal => Stat::EffectInflicted,
        Effect::SpeedDownII => Stat::EffectInflicted,
        Effect::SpeedUpI => Stat::EffectInflicted,
    }
}