use crate::data::effect::Effect;
use crate::wave::heroes::{BasicAttack, Cooldown};
use crate::{
    data::skill::{Select, Skill, SkillType},
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct InspiringAttack {
    pub cooldown: u32,
    pub attack_damage_ratio: f32,
    pub reduce_defense_chance: f32,
    pub reduce_defense_turns: u32,
}

impl Default for InspiringAttack {
    fn default() -> Self {
        Self {
            cooldown: 0,
            attack_damage_ratio: 1.0,
            reduce_defense_chance: 0.7,
            reduce_defense_turns: 2,
        }
    }
}

impl InspiringAttack {
    pub const TYPE: SkillType = SkillType::Basic;
    pub const SELECT: Select = Select::SingleEnemy;

    pub fn execute(
        &self,
        wave: &mut Wave,
        skill: &Skill,
        actor: InstanceIndex,
        defender: InstanceIndex,
    ) {
        wave.attack_single(
            actor,
            defender,
            self.attack_damage_ratio * wave.get_attack(actor),
            skill,
        );

        wave.inflict_single(
            actor,
            defender,
            Effect::DefenseDownII,
            self.reduce_defense_chance,
            self.reduce_defense_turns,
        );

        if let Some(high) = wave.find_highest_attack_alive_ally(actor) {
            wave.attack_single(
                high,
                defender,
                wave.get_attack(high),
                &Skill::BasicAttack(BasicAttack::default()),
            );
        }
    }
}
