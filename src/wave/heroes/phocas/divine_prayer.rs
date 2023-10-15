use crate::wave::heroes::{Cooldown, Execute, Selector, Skilled, Typed};
use crate::{
    data::{
        effect::Effect,
        skill::{Select, Skill, SkillType},
    },
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct DivinePrayer {
    pub cooldown: u32,
}

impl Default for DivinePrayer {
    fn default() -> Self {
        Self { cooldown: 5 }
    }
}

impl Execute for DivinePrayer {
    fn execute(
        &self,
        wave: &mut Wave,
        _skill: &Skill,
        attacker: InstanceIndex,
        _defender: InstanceIndex,
    ) {
        for h in wave.get_ally_indices(attacker) {
            wave.increase_turn_meter_ratio(attacker, h, 1.0);
            wave.reset_all_skills(attacker, h);
            wave.inflict_single(attacker, h, Effect::AttackUpII, 1.0, 1);
        }
    }
}

impl Typed for DivinePrayer {
    const TYPE: SkillType = SkillType::Active;
}

impl Selector for DivinePrayer {
    const SELECT: Select = Select::AllAllies;
}

impl Skilled for DivinePrayer {}
