use crate::wave::heroes::Cooldown;
use crate::{
    data::{
        effect::Effect,
        skill::{Select, Skill, SkillType},
    },
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct ScorchedSoul {
    pub cooldown: u32,
    pub attack_damage_ratio: f32,
    pub hp_burning_chance: f32,
    pub hp_burning_turns: u32,
}

impl Default for ScorchedSoul {
    fn default() -> Self {
        Self {
            cooldown: 4,
            attack_damage_ratio: 1.8,
            hp_burning_chance: 0.3,
            hp_burning_turns: 2,
        }
    }
}

impl ScorchedSoul {
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
            Effect::HPBurning,
            self.hp_burning_chance,
            self.hp_burning_turns,
        );
    }
}
