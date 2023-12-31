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
    BlockBuff,
    BlockDebuff,
    Burn,

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
    DivineLight,
    DivineShield,
    DivineDust,

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
    HPUpI,
    HPUpII,
    HPBurning,

    InferiorSevereWound,
    Immortal,
    Imprison,

    OverflowingLight,

    Poison,

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
    FlowingRainbow,

    // Skill
    ToxicSwamp,
    ForceOfMercy,

    //Faction
    FactionHiddenWaveAttack, // Counter for faction hidden wave
    FactionHiddenWaveSkill,  // Counter for faction hidden wave

    //Util
    _DeepTrapCounter,
    _ElectronTransferCounter,
}

pub enum EffectCategory {
    Buff,
    AttributeBuff,
    Debuff,
    Dot,
    Control,
    AttributeDebuff,
    Unique,
    Util,
}

impl Effect {}

impl Effect {
    pub fn can_be_stacked(&self) -> bool {
        match *self {
            Effect::DivineShield => false,
            _ => true,
        }
    }

    pub fn can_be_removed(&self) -> bool {
        match *self {
            Effect::DivineShield => false,
            _ => true,
        }
    }

    pub fn can_be_stolen(&self) -> bool {
        match *self {
            Effect::DivineShield => false,
            _ => true,
        }
    }

    pub fn get_category(&self) -> EffectCategory {
        match *self {
            Effect::FactionHiddenWaveAttack
            | Effect::FactionHiddenWaveSkill
            | Effect::_DeepTrapCounter
            | Effect::_ElectronTransferCounter
            | Effect::None => EffectCategory::Util,

            Effect::AttackDownI
            | Effect::ColdI
            | Effect::ColdII
            | Effect::ColdIII
            | Effect::CritRateDownI
            | Effect::CritRateDownII
            | Effect::CritDamageDownI
            | Effect::CritDamageDownII
            | Effect::DefenseDownI
            | Effect::DefenseDownII
            | Effect::EffectHitDownI
            | Effect::EffectHitDownII
            | Effect::EffectResistanceDownI
            | Effect::EffectResistanceDownII
            | Effect::SpeedDownI
            | Effect::SpeedDownII
            | Effect::TenacityDownI
            | Effect::TenacityDownII
            | Effect::AttackDownII => EffectCategory::AttributeDebuff,

            Effect::AttackUpI
            | Effect::CritRateUpI
            | Effect::CritRateUpII
            | Effect::CritDamageUpI
            | Effect::CritDamageUpII
            | Effect::DefenseUpI
            | Effect::DefenseUpII
            | Effect::EffectHitUpI
            | Effect::EffectHitUpII
            | Effect::EffectResistanceUpI
            | Effect::EffectResistanceUpII
            | Effect::RippleII
            | Effect::SpeedUpI
            | Effect::SpeedUpII
            | Effect::TenacityUpI
            | Effect::TenacityUpII
            | Effect::HPUpI
            | Effect::HPUpII
            | Effect::AttackUpII => EffectCategory::AttributeBuff,

            Effect::Heal
            | Effect::Immortal
            | Effect::BlockRemoval
            | Effect::Stealth
            | Effect::Counterattack
            | Effect::DamageImmunity
            | Effect::ControlImmunity
            | Effect::ConsolidationI
            | Effect::ConsolidationII
            | Effect::BlockDebuff => EffectCategory::Buff,

            Effect::Silence | Effect::Freeze | Effect::Stun | Effect::Imprison => {
                EffectCategory::Control
            }

            Effect::FeeblenessI
            | Effect::SevereWound
            | Effect::Suffocated
            | Effect::InferiorSevereWound
            | Effect::BlockBuff
            | Effect::WetI
            | Effect::WetII
            | Effect::FeeblenessII => EffectCategory::Debuff,

            Effect::HPBurning | Effect::Poison | Effect::Burn | Effect::Bleed => {
                EffectCategory::Dot
            }

            Effect::ScarletSakura
            | Effect::Arcane
            | Effect::CountessKiss
            | Effect::Blade
            | Effect::FishShoal
            | Effect::RosePoison
            | Effect::ToxicSwamp
            | Effect::ForceOfMercy
            | Effect::DivineLight
            | Effect::DivineShield
            | Effect::DivineDust
            | Effect::OverflowingLight
            | Effect::FlowingRainbow
            | Effect::DeepPoison => EffectCategory::Unique,
        }
    }

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
            Effect::DivineDust => 100,
            Effect::FlowingRainbow => 10,
            Effect::None => 0,
            Effect::AttackUpI
            | Effect::DivineLight
            | Effect::DivineShield
            | Effect::OverflowingLight
            | Effect::HPUpI
            | Effect::HPUpII
            | Effect::Silence
            | Effect::BlockDebuff
            | Effect::InferiorSevereWound
            | Effect::SevereWound
            | Effect::AttackUpII
            | Effect::AttackDownI
            | Effect::AttackDownII
            | Effect::BlockBuff
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
            | Effect::Stun
            | Effect::Imprison
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
            | Effect::_ElectronTransferCounter
            | Effect::_DeepTrapCounter => 99999,
        }
    }

    pub fn is_buff(&self) -> bool {
        match self.get_category() {
            EffectCategory::Buff => true,
            EffectCategory::AttributeBuff => true,
            EffectCategory::Debuff => false,
            EffectCategory::Dot => false,
            EffectCategory::Control => false,
            EffectCategory::AttributeDebuff => false,
            EffectCategory::Unique => false,
            EffectCategory::Util => false,
        }
    }

    pub fn is_unique(&self) -> bool {
        match self.get_category() {
            EffectCategory::Buff => false,
            EffectCategory::AttributeBuff => false,
            EffectCategory::Debuff => false,
            EffectCategory::Dot => false,
            EffectCategory::Control => false,
            EffectCategory::AttributeDebuff => false,
            EffectCategory::Unique => true,
            EffectCategory::Util => false,
        }
    }

    pub fn is_attribute_buff(&self) -> bool {
        match self.get_category() {
            EffectCategory::Buff => false,
            EffectCategory::AttributeBuff => true,
            EffectCategory::Debuff => false,
            EffectCategory::Dot => false,
            EffectCategory::Control => false,
            EffectCategory::AttributeDebuff => false,
            EffectCategory::Unique => false,
            EffectCategory::Util => false,
        }
    }

    pub fn is_attribute_debuff(&self) -> bool {
        match self.get_category() {
            EffectCategory::Buff => false,
            EffectCategory::AttributeBuff => false,
            EffectCategory::Debuff => false,
            EffectCategory::Dot => false,
            EffectCategory::Control => false,
            EffectCategory::AttributeDebuff => true,
            EffectCategory::Unique => false,
            EffectCategory::Util => false,
        }
    }

    pub fn is_debuff(&self) -> bool {
        match self.get_category() {
            EffectCategory::Buff => false,
            EffectCategory::AttributeBuff => false,
            EffectCategory::Debuff => true,
            EffectCategory::Dot => true,
            EffectCategory::Control => true,
            EffectCategory::AttributeDebuff => true,
            EffectCategory::Unique => false,
            EffectCategory::Util => false,
        }
    }

    pub fn is_control(&self) -> bool {
        match self.get_category() {
            EffectCategory::Buff => false,
            EffectCategory::AttributeBuff => false,
            EffectCategory::Debuff => false,
            EffectCategory::Dot => false,
            EffectCategory::Control => true,
            EffectCategory::AttributeDebuff => false,
            EffectCategory::Unique => false,
            EffectCategory::Util => false,
        }
    }

    pub fn is_dot(&self) -> bool {
        match self.get_category() {
            EffectCategory::Buff => false,
            EffectCategory::AttributeBuff => false,
            EffectCategory::Debuff => false,
            EffectCategory::Dot => true,
            EffectCategory::Control => false,
            EffectCategory::AttributeDebuff => false,
            EffectCategory::Unique => false,
            EffectCategory::Util => false,
        }
    }

    pub fn get_reduction(&self) -> u32 {
        // TODO plug in extra cases
        match self.get_category() {
            EffectCategory::Buff => 1,
            EffectCategory::AttributeBuff => 1,
            EffectCategory::Debuff => 1,
            EffectCategory::Dot => 1,
            EffectCategory::Control => 1,
            EffectCategory::AttributeDebuff => 1,
            EffectCategory::Unique => 0,
            EffectCategory::Util => 1,
        }
    }
}
