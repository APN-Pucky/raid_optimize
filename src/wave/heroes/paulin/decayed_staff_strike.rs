use crate::data::effect::Effect;
use crate::roll;
use crate::wave::heroes::{BasicAttack, Cooldown};
use crate::{
    data::skill::{Select, Skill, SkillType},
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct DecayedStaffStrike {
    pub cooldown: u32,
    pub remove_chance: f32,
    pub attack_damage_ratio: f32,
}

impl Default for DecayedStaffStrike {
    fn default() -> Self {
        Self {
            cooldown: 0,
            remove_chance: 0.6,
            attack_damage_ratio: 1.2,
        }
    }
}

impl DecayedStaffStrike {
    pub const TYPE: SkillType = SkillType::Basic;
    pub const SELECT: Select = Select::SingleEnemy;

    pub fn execute(
        &self,
        wave: &mut Wave,
        skill: &Skill,
        actor: InstanceIndex,
        defender: InstanceIndex,
    ) {
        if roll(self.remove_chance) {
            if wave.get_effect_resistance(actor) < 1.0 {
                for t in wave.get_enemies_indices(actor) {
                    wave.remove_one_random_effect_filter_single(
                        actor,
                        t,
                        Effect::is_attribute_buff,
                    );
                }
            } else {
                for t in wave.get_enemies_indices(actor) {
                    wave.remove_one_random_effect_filter_single(actor, t, Effect::is_buff);
                }
            }
        }

        wave.attack_single(
            actor,
            defender,
            self.attack_damage_ratio * wave.get_attack(actor),
            skill,
        );
    }
}
