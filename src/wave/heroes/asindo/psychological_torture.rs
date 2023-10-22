use crate::data::effect::Effect;
use crate::wave::heroes::Cooldown;
use crate::{
    data::skill::{Select, Skill, SkillType},
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct PsychologicalTorture {
    pub cooldown: u32,
    pub attack_damage_ratio: f32,
    pub silence_chance: f32,
    pub silence_turns: u32,
    pub turn_meter_reduction_ratio: f32,
}

impl Default for PsychologicalTorture {
    fn default() -> Self {
        Self {
            cooldown: 0,
            attack_damage_ratio: 2.0,
            silence_chance: 0.8,
            silence_turns: 1,
            turn_meter_reduction_ratio: 0.15,
        }
    }
}

impl PsychologicalTorture {
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
        if wave.has_effect(defender, Effect::Silence) {
            wave.reduce_turn_meter_ratio(attacker, defender, self.turn_meter_reduction_ratio);
        } else {
            wave.inflict_single(
                attacker,
                defender,
                Effect::Silence,
                self.silence_chance,
                self.silence_turns,
            );
        }
    }
}
