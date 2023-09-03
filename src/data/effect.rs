use enum_map::Enum;

#[derive(Debug,Enum, PartialEq, Eq ,strum_macros::Display,Copy,Clone,Deserialize, )]
pub enum Effect {
    WetI,
    WetII,
    ColdI,
    ColdII,
    ColdIII,
    Bleed,
    Heal,
    HPBurning,
    Suffocated,
    BlockDebuff,
    EffectResistanceUpI,
    EffectResistanceUpII,
    EffectResistanceDownI,
    EffectResistanceDownII,
    RippleII,
    AttackUpI,
    AttackUpII,
    AttackDownI,
    AttackDownII,
    TenacityUpI,
    TenacityUpII,
    TenacityDownI,
    TenacityDownII,
    SpeedDownI,
    SpeedDownII,
    SpeedUpI,
    EffectHitUpI,
    EffectHitUpII,
    EffectHitDownI,
    EffectHitDownII,
    DefenseUpI,
    DefenseUpII,
    DefenseDownI,
    DefenseDownII,
    CritRateUpI,
    CritRateUpII,
    CritRateDownI,
    CritRateDownII,
    CritDamageDownI,
    CritDamageDownII,
    CritDamageUpI,
    CritDamageUpII,

    
    //Debuff
    BlockBuff,
    Stun,
    //DotDebuff
    Poison,
    Burn,
    //Buff
    Stealth,
    CounterAttack,
    DamageImmunity,
    ControlImmunity, // Needs implementation
    //Unique
    ScarletSakura,
    Arcane,
    Blade,

    //Faction
    FactionHiddenWaveAttack, // Counter for faction hidden wave
    FactionHiddenWaveSkill, // Counter for faction hidden wave
    None,
}

pub fn get_max(effect:Effect) -> u32 {
    match effect {
        Effect::ScarletSakura => 20,
        Effect::Arcane => 5,
        Effect::Bleed => 10,
        Effect::HPBurning => 5,
        Effect::FactionHiddenWaveAttack => 2,
        Effect::FactionHiddenWaveSkill => 2,
        _ => 999999
    }
}

pub fn is_buff(effect : Effect) -> bool {
    ! is_debuff(effect)
}

pub fn is_attribute_debuff(effect : Effect) -> bool {
    match effect {
        Effect::WetI => true,
        Effect::WetII => true,
        Effect::ColdI => true,
        Effect::ColdII => true,
        Effect::ColdIII => true,
        Effect::SpeedDownI => true,
        Effect::SpeedDownII => true,
        Effect::EffectResistanceDownI => true,
        Effect::EffectResistanceDownII => true,
        Effect::AttackDownI => true,
        Effect::AttackDownII => true,
        Effect::TenacityDownI => true,
        Effect::TenacityDownII => true,
        Effect::SpeedDownI => true,
        Effect::SpeedDownII => true,
        Effect::EffectHitDownI => true,
        Effect::EffectHitDownII => true,
        Effect::DefenseDownI => true,
        Effect::DefenseDownII => true,
        Effect::CritRateDownI => true,
        Effect::CritRateDownII => true,
        Effect::CritDamageDownI => true,
        Effect::CritDamageDownII => true,

        Effect::Poison=> false,
        Effect::Burn=> false,
        Effect::Bleed => false,
        Effect::HPBurning=> false,
        _ => false,
    }
}

pub fn is_debuff(effect:Effect) -> bool {
    match effect {
        Effect::WetI => true,
        Effect::WetII => true,
        Effect::ColdI => true,
        Effect::ColdII => true,
        Effect::ColdIII => true,
        Effect::Bleed => true,
        Effect::HPBurning => true,
        Effect::Suffocated => true,
        Effect::EffectResistanceDownII => true,
        Effect::AttackDownI => true,
        Effect::AttackDownII => true,
        Effect::TenacityDownI => true,
        Effect::TenacityDownII => true,
        Effect::SpeedDownI => true,
        Effect::SpeedDownII => true,
        Effect::EffectHitDownI => true,
        Effect::EffectHitDownII => true,
        Effect::DefenseDownI => true,
        Effect::DefenseDownII => true,
        Effect::CritRateDownI => true,
        Effect::CritRateDownII => true,
        Effect::CritDamageDownI => true,
        Effect::CritDamageDownII => true,
        _ => false,
    }
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
        Effect::BlockDebuff => false,
        Effect::EffectResistanceDownII => false,
        Effect::RippleII => false,
        Effect::AttackUpII => false,
        Effect::Heal => false,
        Effect::SpeedDownII => false,
        Effect::SpeedUpI => false,
        Effect::None => false,
        _ => false,
    }
}