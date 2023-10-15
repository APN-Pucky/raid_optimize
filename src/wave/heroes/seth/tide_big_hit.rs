use crate::wave::heroes::Cooldown;
use crate::{
    data::{
        effect::Effect,
        skill::{Select, Skill, SkillType},
    },
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct TideBigHit {
    pub cooldown: u32,
    pub max_hp_damage_ratio: f32,
    pub suffocated_chance: f32,
    pub suffocated_turns: u32,
}

impl Default for TideBigHit {
    fn default() -> Self {
        Self {
            cooldown: 0,
            max_hp_damage_ratio: 0.18,
            suffocated_chance: 0.1,
            suffocated_turns: 1,
        }
    }
}

impl TideBigHit {
    pub const TYPE: SkillType = SkillType::Basic;
    pub const SELECT: Select = Select::AllEnemies;
    pub fn execute(
        &self,
        wave: &mut Wave,
        skill: &Skill,
        attacker: InstanceIndex,
        defender: InstanceIndex,
    ) {
        log::debug!("{} uses Tide Big Hit on {}", attacker, defender);
        let mut chance = self.suffocated_chance;
        wave.attack_single(
            attacker,
            defender,
            wave.get_max_health(attacker) * self.max_hp_damage_ratio,
            skill,
        );
        if wave.has_effect(defender, Effect::WetI)
            || wave.has_effect(defender, Effect::WetII)
            || wave.has_effect(defender, Effect::ColdI)
            || wave.has_effect(defender, Effect::ColdII)
        {
            log::debug!("{} is wet or cold +15% suffocation chance", defender);
            chance += 0.15;
        }
        wave.inflict_single(
            attacker,
            defender,
            Effect::Suffocated,
            chance,
            self.suffocated_turns,
        );
    }
}
