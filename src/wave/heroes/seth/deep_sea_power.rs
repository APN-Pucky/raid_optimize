use crate::wave::heroes::Cooldown;
use crate::{
    data::{
        effect::Effect,
        skill::{Select, Skill, SkillType},
    },
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct DeepSeaPower {
    pub cooldown: u32,
    pub max_hp_shield_ratio: f32,
    pub shield_turns: u32,
    pub tenacity_increase_turns: u32,
}

impl Default for DeepSeaPower {
    fn default() -> Self {
        Self {
            cooldown: 5,
            max_hp_shield_ratio: 0.25,
            shield_turns: 2,
            tenacity_increase_turns: 2,
        }
    }
}

impl DeepSeaPower {
    pub const TYPE: SkillType = SkillType::Active;
    pub const SELECT: Select = Select::AllEnemies;
    pub fn execute(
        &self,
        wave: &mut Wave,
        _skill: &Skill,
        actor: InstanceIndex,
        _defender: InstanceIndex,
    ) {
        let max_hp = wave.get_max_health(actor);
        wave.shield_ally_team(actor, self.max_hp_shield_ratio * max_hp, self.shield_turns);
        wave.inflict_ally_team(
            actor,
            Effect::TenacityUpII,
            1.0,
            self.tenacity_increase_turns,
        );
    }
}
