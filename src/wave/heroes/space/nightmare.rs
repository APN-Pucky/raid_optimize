use crate::wave::heroes::Cooldown;
use crate::{
    data::{
        effect::Effect,
        skill::{Select, Skill, SkillType},
    },
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct Nightmare {
    pub cooldown: u32,
    pub attack_damage_ratio: f32,
    pub reduce_speed_chance: f32,
    pub reduce_speed_turns: u32,
    pub increase_speed_turns: u32,
}

impl Default for Nightmare {
    fn default() -> Self {
        Self {
            cooldown: 4,
            attack_damage_ratio: 2.0,
            reduce_speed_chance: 0.6,
            reduce_speed_turns: 2,
            increase_speed_turns: 2,
        }
    }
}

impl Nightmare {
    pub const TYPE: SkillType = SkillType::Active;
    pub const SELECT: Select = Select::AllEnemies;
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
            Effect::SpeedDownII,
            self.reduce_speed_chance,
            self.reduce_speed_turns,
        );
        wave.inflict_ally_team(attacker, Effect::SpeedUpI, 1.0, self.increase_speed_turns);
        //TODO target make no sense here
        //attacker.inflict(defender,Effect::SpeedUpI, 1.0, increase_speed_turns);
    }
}
