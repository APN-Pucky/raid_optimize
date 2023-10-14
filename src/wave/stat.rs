use enum_map::{Enum, EnumMap};

use crate::data::effect::Effect;

use super::{ Wave, InstanceIndex};

#[derive(Debug,Enum, PartialEq, Eq, strum_macros::Display,Copy,Clone)]
pub enum Stat {
    Attacks,
    Attacked,
    Attack,

    Blocked,
    BlockedBy,

    CriticalStrikes,
    CriticalStriked,
    CriticalDamage,
    CriticalDamaged,

    DamageReflected,
    DamageReflecteded,
    DamageDone,
    DamageTaken,
    Defends,

    EffectInflicted,

    HealthHealed,
    HealthRestored,
    HealthLost,

    LostToTenacity,
    Leeched,
    LeechedOf,

    Piercing,
    Pierced,


    Revives,
    Revived,
    ResetSkill,

    SavedByTenacity,
    Shielded,
    ShieldBlocked,
    TurnMeterReduced,
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
        Effect::BlockDebuff => Stat::EffectInflicted,
        Effect::EffectResistanceDownII => Stat::EffectInflicted,
        Effect::RippleII => Stat::EffectInflicted,
        Effect::AttackUpII => Stat::EffectInflicted,
        Effect::TenacityUpII => Stat::EffectInflicted,
        Effect::Heal => Stat::EffectInflicted,
        Effect::SpeedDownI => Stat::EffectInflicted,
        Effect::SpeedDownII => Stat::EffectInflicted,
        Effect::SpeedUpI => Stat::EffectInflicted,
        Effect::None => Stat::EffectInflicted,
        _ => Stat::EffectInflicted,
    }
}

impl Wave<'_> {
    pub fn add_stat(&mut self, actor:InstanceIndex, key: Stat, statistics: f32 ) {
        if self.track_statistics {
            self.statistics[actor].sts[key] += statistics;
        }
    }
}

#[derive(Debug)]
pub struct Statistics {
    pub sts :  EnumMap<Stat,f32>,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            sts : EnumMap::default(),
        }
    }
    pub fn clear(&mut self) {
        for (_key,value) in self.sts.iter_mut() {
            *value = 0.0;
        }
    }
}