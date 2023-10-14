use enum_map::Enum;

use strum_macros::EnumIter;


#[derive(EnumIter, Debug,Enum, PartialEq, Eq ,strum_macros::Display,Copy,Clone,Deserialize, Serialize, )]
pub enum Effect {
    None,

    AttackUpI,
    AttackUpII,
    AttackDownI,
    AttackDownII,

    Bleed,
    BlockDebuff,

    ColdI,
    ColdII,
    ColdIII,
    CritRateUpI,
    CritRateUpII,
    CritRateDownI,
    CritRateDownII,
    CritDamageDownI,
    CritDamageDownII,
    CritDamageUpI,
    CritDamageUpII,

    DeepPoison,

    DefenseUpI,
    DefenseUpII,
    DefenseDownI,
    DefenseDownII,

    EffectHitUpI,
    EffectHitUpII,
    EffectHitDownI,
    EffectHitDownII,

    EffectResistanceUpI,
    EffectResistanceUpII,
    EffectResistanceDownI,
    EffectResistanceDownII,

    FeeblenessI,
    FeeblenessII,

    Heal,
    HPBurning,
    
    Immortal,

    RippleII,

    Suffocated,


    SpeedDownI,
    SpeedDownII,
    SpeedUpI,
    SpeedUpII,

    TenacityUpI,
    TenacityUpII,
    TenacityDownI,
    TenacityDownII,


    WetI,
    WetII,

    
    //Debuff
    BlockBuff,
    Stun,
    //DotDebuff
    Poison,
    Burn,
    //Buff
    BlockRemoval,
    Stealth,
    Counterattack,
    DamageImmunity,
    ControlImmunity, // Needs implementation
    //AttributeBuff
    ConsolidationI,
    ConsolidationII,
    //Unique
    ScarletSakura,
    Arcane,
    Blade,
    FishShoal,
    RosePoison,

    // Skill
    ToxicSwamp,
    ForceOfMercy,

    //Faction
    FactionHiddenWaveAttack, // Counter for faction hidden wave
    FactionHiddenWaveSkill, // Counter for faction hidden wave

    //Util
    _DeepTrapCounter
}

pub fn get_max(effect:Effect) -> u32 {
    match effect {
        Effect::ScarletSakura => 20,
        Effect::Arcane => 5,
        Effect::Bleed => 10,
        Effect::HPBurning => 5,
        Effect::FactionHiddenWaveAttack => 2,
        Effect::FactionHiddenWaveSkill => 2,
        Effect::FishShoal => 3,
        Effect::RosePoison => 40,
        Effect::Poison => 10,
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
