use crate::wave::heroes::Cooldown;
use crate::{
    data::{
        effect::Effect,
        skill::{Select, Skill, SkillType},
    },
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct ScytheStrike {
    pub cooldown: u32,
    pub attack_damage_ratio: f32,
    pub bleed_chance: f32,
    pub bleed_turns: u32,
}

impl Default for ScytheStrike {
    fn default() -> Self {
        Self {
            cooldown: 0,
            attack_damage_ratio: 1.4,
            bleed_chance: 0.8,
            bleed_turns: 2,
        }
    }
}

impl ScytheStrike {
    pub const TYPE: SkillType = SkillType::Basic;
    pub const SELECT: Select = Select::SingleEnemy;

    pub fn execute(
        &self,
        wave: &mut Wave,
        skill: &Skill,
        attacker: InstanceIndex,
        defender: InstanceIndex,
    ) {
        wave.attack_single(
            attacker,
            defender,
            wave.get_attack_damage(attacker) * self.attack_damage_ratio,
            skill,
        );
        wave.inflict_single(
            attacker,
            defender,
            Effect::Bleed,
            self.bleed_chance,
            self.bleed_turns,
        );
    }
}
