use crate::wave::heroes::{Cooldown, Execute, Selector, Skilled, Typed};
use crate::{
    data::{
        effect::Effect,
        skill::{Select, Skill, SkillType},
    },
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct Curse {
    pub cooldown: u32,
    pub attack_damage_ratio: f32,
    pub poison_chance: f32,
    pub poison_turns: u32,
    pub restore_max_health_per_poison: f32,
}

impl Default for Curse {
    fn default() -> Self {
        Self {
            cooldown: 0,
            attack_damage_ratio: 3.2,
            poison_chance: 1.0,
            poison_turns: 2,
            restore_max_health_per_poison: 0.025,
        }
    }
}

impl Execute for Curse {
    fn execute(
        &self,
        wave: &mut Wave,
        skill: &Skill,
        attacker: InstanceIndex,
        defender: InstanceIndex,
    ) {
        let actor = attacker;
        wave.attack_single(
            attacker,
            defender,
            wave.get_attack_damage(attacker) * self.attack_damage_ratio,
            skill,
        );
        wave.inflict_single(
            actor,
            defender,
            Effect::Poison,
            self.poison_chance,
            self.poison_turns,
        );
        wave.restore_single(
            actor,
            actor,
            wave.get_max_health(actor)
                * self.restore_max_health_per_poison
                * (wave.effects[defender].get(Effect::Poison) as f32),
        );
    }
}

impl Typed for Curse {
    const TYPE: SkillType = SkillType::Basic;
}

impl Selector for Curse {
    const SELECT: Select = Select::SingleEnemy;
}

impl Skilled for Curse {}
