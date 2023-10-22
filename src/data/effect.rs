use enum_map::Enum;

use strum_macros::EnumIter;

pub type EffectFilter = fn(&Effect) -> bool;

#[derive(
    EnumIter, Debug, Enum, PartialEq, Eq, strum_macros::Display, Copy, Clone, Deserialize, Serialize,
)]
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
    Freeze,

    Heal,
    HPBurning,

    InferiorSevereWound,
    Immortal,

    RippleII,

    SevereWound,
    Silence,
    SpeedDownI,
    SpeedDownII,
    SpeedUpI,
    SpeedUpII,
    Suffocated,
    Stun,

    TenacityUpI,
    TenacityUpII,
    TenacityDownI,
    TenacityDownII,

    WetI,
    WetII,

    //Debuff
    BlockBuff,
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
    CountessKiss,
    Blade,
    FishShoal,
    RosePoison,

    // Skill
    ToxicSwamp,
    ForceOfMercy,

    //Faction
    FactionHiddenWaveAttack, // Counter for faction hidden wave
    FactionHiddenWaveSkill,  // Counter for faction hidden wave

    //Util
    _DeepTrapCounter,
}

impl Effect {
    pub fn get_max(&self) -> u32 {
        match *self {
            Effect::ScarletSakura => 20,
            Effect::Arcane => 5,
            Effect::Bleed => 10,
            Effect::HPBurning => 5,
            Effect::FactionHiddenWaveAttack => 2,
            Effect::FactionHiddenWaveSkill => 2,
            Effect::FishShoal => 3,
            Effect::RosePoison => 40,
            Effect::Poison => 10,
            Effect::CountessKiss => 10,
            Effect::None => 0,
            Effect::AttackUpI
            | Effect::Silence
            | Effect::InferiorSevereWound
            | Effect::SevereWound
            | Effect::AttackUpII
            | Effect::AttackDownI
            | Effect::AttackDownII
            | Effect::BlockDebuff
            | Effect::ColdI
            | Effect::ColdII
            | Effect::ColdIII
            | Effect::CritRateUpI
            | Effect::CritRateUpII
            | Effect::CritRateDownI
            | Effect::CritRateDownII
            | Effect::CritDamageDownI
            | Effect::CritDamageDownII
            | Effect::CritDamageUpI
            | Effect::CritDamageUpII
            | Effect::DeepPoison
            | Effect::DefenseUpI
            | Effect::DefenseUpII
            | Effect::DefenseDownI
            | Effect::DefenseDownII
            | Effect::EffectHitUpI
            | Effect::EffectHitUpII
            | Effect::EffectHitDownI
            | Effect::EffectHitDownII
            | Effect::EffectResistanceUpI
            | Effect::EffectResistanceUpII
            | Effect::EffectResistanceDownI
            | Effect::EffectResistanceDownII
            | Effect::FeeblenessI
            | Effect::FeeblenessII
            | Effect::Heal
            | Effect::Immortal
            | Effect::RippleII
            | Effect::Suffocated
            | Effect::SpeedDownI
            | Effect::SpeedDownII
            | Effect::SpeedUpI
            | Effect::SpeedUpII
            | Effect::TenacityUpI
            | Effect::TenacityUpII
            | Effect::TenacityDownI
            | Effect::TenacityDownII
            | Effect::WetI
            | Effect::WetII
            | Effect::BlockBuff
            | Effect::Stun
            | Effect::Freeze
            | Effect::Burn
            | Effect::BlockRemoval
            | Effect::Stealth
            | Effect::Counterattack
            | Effect::DamageImmunity
            | Effect::ControlImmunity
            | Effect::ConsolidationI
            | Effect::ConsolidationII
            | Effect::Blade
            | Effect::ToxicSwamp
            | Effect::ForceOfMercy
            | Effect::_DeepTrapCounter => 99999,
        }
    }

    pub fn is_buff(&self) -> bool {
        !self.is_debuff()
    }

    pub fn is_unique(&self) -> bool {
        match self {
            Effect::ScarletSakura | Effect::DeepPoison | Effect::Blade | Effect::CountessKiss => {
                true
            }

            Effect::None
            | Effect::Silence
            | Effect::InferiorSevereWound
            | Effect::SevereWound
            | Effect::AttackUpI
            | Effect::AttackUpII
            | Effect::AttackDownI
            | Effect::AttackDownII
            | Effect::Bleed
            | Effect::BlockDebuff
            | Effect::ColdI
            | Effect::ColdII
            | Effect::ColdIII
            | Effect::CritRateUpI
            | Effect::CritRateUpII
            | Effect::CritRateDownI
            | Effect::CritRateDownII
            | Effect::CritDamageDownI
            | Effect::CritDamageDownII
            | Effect::CritDamageUpI
            | Effect::CritDamageUpII
            | Effect::DefenseUpI
            | Effect::DefenseUpII
            | Effect::DefenseDownI
            | Effect::DefenseDownII
            | Effect::EffectHitUpI
            | Effect::EffectHitUpII
            | Effect::EffectHitDownI
            | Effect::EffectHitDownII
            | Effect::EffectResistanceUpI
            | Effect::EffectResistanceUpII
            | Effect::EffectResistanceDownI
            | Effect::EffectResistanceDownII
            | Effect::FeeblenessI
            | Effect::FeeblenessII
            | Effect::Heal
            | Effect::HPBurning
            | Effect::Immortal
            | Effect::RippleII
            | Effect::Suffocated
            | Effect::SpeedDownI
            | Effect::SpeedDownII
            | Effect::SpeedUpI
            | Effect::SpeedUpII
            | Effect::TenacityUpI
            | Effect::TenacityUpII
            | Effect::TenacityDownI
            | Effect::TenacityDownII
            | Effect::WetI
            | Effect::WetII
            | Effect::BlockBuff
            | Effect::Stun
            | Effect::Freeze
            | Effect::Poison
            | Effect::Burn
            | Effect::BlockRemoval
            | Effect::Stealth
            | Effect::Counterattack
            | Effect::DamageImmunity
            | Effect::ControlImmunity
            | Effect::ConsolidationI
            | Effect::ConsolidationII
            | Effect::Arcane
            | Effect::FishShoal
            | Effect::RosePoison
            | Effect::ToxicSwamp
            | Effect::ForceOfMercy
            | Effect::FactionHiddenWaveAttack
            | Effect::FactionHiddenWaveSkill
            | Effect::_DeepTrapCounter => false,
        }
    }

    pub fn is_attribute_debuff(&self) -> bool {
        match self {
            Effect::WetI
            | Effect::WetII
            | Effect::ColdI
            | Effect::ColdII
            | Effect::ColdIII
            | Effect::SpeedDownI
            | Effect::SpeedDownII
            | Effect::EffectResistanceDownI
            | Effect::EffectResistanceDownII
            | Effect::AttackDownI
            | Effect::AttackDownII
            | Effect::TenacityDownI
            | Effect::TenacityDownII
            | Effect::EffectHitDownI
            | Effect::EffectHitDownII
            | Effect::DefenseDownI
            | Effect::DefenseDownII
            | Effect::CritRateDownI
            | Effect::CritRateDownII
            | Effect::CritDamageDownI
            | Effect::CritDamageDownII => true,

            Effect::Poison
            | Effect::Silence
            | Effect::InferiorSevereWound
            | Effect::SevereWound
            | Effect::Burn
            | Effect::Bleed
            | Effect::HPBurning
            | Effect::None
            | Effect::AttackUpI
            | Effect::AttackUpII
            | Effect::BlockDebuff
            | Effect::CritRateUpI
            | Effect::CritRateUpII
            | Effect::CritDamageUpI
            | Effect::CritDamageUpII
            | Effect::DeepPoison
            | Effect::DefenseUpI
            | Effect::DefenseUpII
            | Effect::EffectHitUpI
            | Effect::EffectHitUpII
            | Effect::EffectResistanceUpI
            | Effect::EffectResistanceUpII
            | Effect::FeeblenessI
            | Effect::FeeblenessII
            | Effect::Heal
            | Effect::Immortal
            | Effect::RippleII
            | Effect::Suffocated
            | Effect::SpeedUpI
            | Effect::SpeedUpII
            | Effect::TenacityUpI
            | Effect::TenacityUpII
            | Effect::BlockBuff
            | Effect::Stun
            | Effect::Freeze
            | Effect::BlockRemoval
            | Effect::Stealth
            | Effect::Counterattack
            | Effect::DamageImmunity
            | Effect::ControlImmunity
            | Effect::ConsolidationI
            | Effect::ConsolidationII
            | Effect::ScarletSakura
            | Effect::Arcane
            | Effect::CountessKiss
            | Effect::Blade
            | Effect::FishShoal
            | Effect::RosePoison
            | Effect::ToxicSwamp
            | Effect::ForceOfMercy
            | Effect::FactionHiddenWaveAttack
            | Effect::FactionHiddenWaveSkill
            | Effect::_DeepTrapCounter => false,
        }
    }

    pub fn is_debuff(&self) -> bool {
        match self {
            Effect::WetI
            | Effect::Silence
            | Effect::InferiorSevereWound
            | Effect::SevereWound
            | Effect::WetII
            | Effect::ColdI
            | Effect::ColdII
            | Effect::ColdIII
            | Effect::Bleed
            | Effect::HPBurning
            | Effect::Suffocated
            | Effect::EffectResistanceDownI
            | Effect::EffectResistanceDownII
            | Effect::AttackDownI
            | Effect::AttackDownII
            | Effect::TenacityDownI
            | Effect::TenacityDownII
            | Effect::SpeedDownI
            | Effect::SpeedDownII
            | Effect::EffectHitDownI
            | Effect::EffectHitDownII
            | Effect::FeeblenessI
            | Effect::FeeblenessII
            | Effect::DefenseDownI
            | Effect::DefenseDownII
            | Effect::CritRateDownI
            | Effect::CritRateDownII
            | Effect::CritDamageDownI
            | Effect::CritDamageDownII => true,

            Effect::None
            | Effect::AttackUpI
            | Effect::AttackUpII
            | Effect::BlockDebuff
            | Effect::CritRateUpI
            | Effect::CritRateUpII
            | Effect::CritDamageUpI
            | Effect::CritDamageUpII
            | Effect::DeepPoison
            | Effect::DefenseUpI
            | Effect::DefenseUpII
            | Effect::EffectHitUpI
            | Effect::EffectHitUpII
            | Effect::EffectResistanceUpI
            | Effect::EffectResistanceUpII
            | Effect::Heal
            | Effect::Immortal
            | Effect::RippleII
            | Effect::SpeedUpI
            | Effect::SpeedUpII
            | Effect::TenacityUpI
            | Effect::TenacityUpII
            | Effect::BlockBuff
            | Effect::Stun
            | Effect::Freeze
            | Effect::Poison
            | Effect::Burn
            | Effect::BlockRemoval
            | Effect::Stealth
            | Effect::Counterattack
            | Effect::DamageImmunity
            | Effect::ControlImmunity
            | Effect::ConsolidationI
            | Effect::ConsolidationII
            | Effect::ScarletSakura
            | Effect::Arcane
            | Effect::CountessKiss
            | Effect::Blade
            | Effect::FishShoal
            | Effect::RosePoison
            | Effect::ToxicSwamp
            | Effect::ForceOfMercy
            | Effect::FactionHiddenWaveAttack
            | Effect::FactionHiddenWaveSkill
            | Effect::_DeepTrapCounter => false,
        }
    }

    pub fn is_control(&self) -> bool {
        match self {
            Effect::Silence | Effect::Freeze | Effect::Stun => true,

            Effect::WetI
            | Effect::Poison
            | Effect::HPBurning
            | Effect::Bleed
            | Effect::InferiorSevereWound
            | Effect::SevereWound
            | Effect::WetII
            | Effect::ColdI
            | Effect::ColdII
            | Effect::ColdIII
            | Effect::Suffocated
            | Effect::BlockDebuff
            | Effect::EffectResistanceDownII
            | Effect::RippleII
            | Effect::AttackUpII
            | Effect::Heal
            | Effect::SpeedDownII
            | Effect::SpeedUpI
            | Effect::None
            | Effect::AttackUpI
            | Effect::AttackDownI
            | Effect::AttackDownII
            | Effect::CritRateUpI
            | Effect::CritRateUpII
            | Effect::CritRateDownI
            | Effect::CritRateDownII
            | Effect::CritDamageDownI
            | Effect::CritDamageDownII
            | Effect::CritDamageUpI
            | Effect::CritDamageUpII
            | Effect::DeepPoison
            | Effect::DefenseUpI
            | Effect::DefenseUpII
            | Effect::DefenseDownI
            | Effect::DefenseDownII
            | Effect::EffectHitUpI
            | Effect::EffectHitUpII
            | Effect::EffectHitDownI
            | Effect::EffectHitDownII
            | Effect::EffectResistanceUpI
            | Effect::EffectResistanceUpII
            | Effect::EffectResistanceDownI
            | Effect::FeeblenessI
            | Effect::FeeblenessII
            | Effect::Immortal
            | Effect::SpeedDownI
            | Effect::SpeedUpII
            | Effect::TenacityUpI
            | Effect::TenacityUpII
            | Effect::TenacityDownI
            | Effect::TenacityDownII
            | Effect::BlockBuff
            | Effect::Burn
            | Effect::BlockRemoval
            | Effect::Stealth
            | Effect::Counterattack
            | Effect::DamageImmunity
            | Effect::ControlImmunity
            | Effect::ConsolidationI
            | Effect::ConsolidationII
            | Effect::ScarletSakura
            | Effect::Arcane
            | Effect::CountessKiss
            | Effect::Blade
            | Effect::FishShoal
            | Effect::RosePoison
            | Effect::ToxicSwamp
            | Effect::ForceOfMercy
            | Effect::FactionHiddenWaveAttack
            | Effect::FactionHiddenWaveSkill
            | Effect::_DeepTrapCounter => false,
        }
    }

    pub fn is_dot(&self) -> bool {
        match self {
            Effect::Poison | Effect::Bleed | Effect::HPBurning => true,

            Effect::WetI
            | Effect::Silence
            | Effect::InferiorSevereWound
            | Effect::SevereWound
            | Effect::WetII
            | Effect::ColdI
            | Effect::ColdII
            | Effect::ColdIII
            | Effect::Suffocated
            | Effect::BlockDebuff
            | Effect::EffectResistanceDownII
            | Effect::RippleII
            | Effect::AttackUpII
            | Effect::Heal
            | Effect::SpeedDownII
            | Effect::SpeedUpI
            | Effect::None
            | Effect::AttackUpI
            | Effect::AttackDownI
            | Effect::AttackDownII
            | Effect::CritRateUpI
            | Effect::CritRateUpII
            | Effect::CritRateDownI
            | Effect::CritRateDownII
            | Effect::CritDamageDownI
            | Effect::CritDamageDownII
            | Effect::CritDamageUpI
            | Effect::CritDamageUpII
            | Effect::DeepPoison
            | Effect::DefenseUpI
            | Effect::DefenseUpII
            | Effect::DefenseDownI
            | Effect::DefenseDownII
            | Effect::EffectHitUpI
            | Effect::EffectHitUpII
            | Effect::EffectHitDownI
            | Effect::EffectHitDownII
            | Effect::EffectResistanceUpI
            | Effect::EffectResistanceUpII
            | Effect::EffectResistanceDownI
            | Effect::FeeblenessI
            | Effect::FeeblenessII
            | Effect::Immortal
            | Effect::SpeedDownI
            | Effect::SpeedUpII
            | Effect::TenacityUpI
            | Effect::TenacityUpII
            | Effect::TenacityDownI
            | Effect::TenacityDownII
            | Effect::BlockBuff
            | Effect::Stun
            | Effect::Freeze
            | Effect::Burn
            | Effect::BlockRemoval
            | Effect::Stealth
            | Effect::Counterattack
            | Effect::DamageImmunity
            | Effect::ControlImmunity
            | Effect::ConsolidationI
            | Effect::ConsolidationII
            | Effect::ScarletSakura
            | Effect::Arcane
            | Effect::CountessKiss
            | Effect::Blade
            | Effect::FishShoal
            | Effect::RosePoison
            | Effect::ToxicSwamp
            | Effect::ForceOfMercy
            | Effect::FactionHiddenWaveAttack
            | Effect::FactionHiddenWaveSkill
            | Effect::_DeepTrapCounter => false,
        }
    }
}
