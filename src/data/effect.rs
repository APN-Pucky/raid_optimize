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
        WetI => true,
        WetII => true,
        ColdI => true,
        ColdII => true,
        ColdIII => true,
        SpeedDownI => true,
        SpeedDownII => true,
        EffectResistanceDownI => true,
        EffectResistanceDownII => true,
        AttackDownI => true,
        AttackDownII => true,
        TenacityDownI => true,
        TenacityDownII => true,
        SpeedDownI => true,
        SpeedDownII => true,
        EffectHitDownI => true,
        EffectHitDownII => true,
        DefenseDownI => true,
        DefenseDownII => true,
        CritRateDownI => true,
        CritRateDownII => true,
        CritDamageDownI => true,
        CritDamageDownII => true,

        Poison=> false,
        Burn=> false,
        Bleed => false,
        HPBurning=> false,
        _ => false,
    }
}

pub fn is_debuff(effect:Effect) -> bool {
    match effect {
        WetI => true,
        WetII => true,
        ColdI => true,
        ColdII => true,
        ColdIII => true,
        Bleed => true,
        HPBurning => true,
        Suffocated => true,
        EffectResistanceDownII => true,
        AttackDownI => true,
        AttackDownII => true,
        TenacityDownI => true,
        TenacityDownII => true,
        SpeedDownI => true,
        SpeedDownII => true,
        EffectHitDownI => true,
        EffectHitDownII => true,
        DefenseDownI => true,
        DefenseDownII => true,
        CritRateDownI => true,
        CritRateDownII => true,
        CritDamageDownI => true,
        CritDamageDownII => true,
        _ => false,
    }
}

pub fn is_dot(effect : Effect) -> bool {
    match effect {
        WetI => false,
        WetII => false,
        ColdI => false,
        ColdII => false,
        ColdIII => false,
        Bleed => true,
        HPBurning => true,
        Suffocated => false,
        BlockDebuff => false,
        EffectResistanceDownII => false,
        RippleII => false,
        AttackUpII => false,
        Heal => false,
        SpeedDownII => false,
        SpeedUpI => false,
        Effect::None => false,
        _ => false,
    }
}