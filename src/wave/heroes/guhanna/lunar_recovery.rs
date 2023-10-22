use crate::data::effect::Effect;
use crate::wave::heroes::Cooldown;
use crate::{
    data::skill::{Select, Skill, SkillType},
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct LunarRecovery {
    pub cooldown: u32,
    pub restore_hp_max_ratio: f32,
    pub increase_turn_meter_ratio: f32,
    pub increase_attack_chance: f32,
    pub increase_attack_turns: u32,
    pub increase_restoration_hp_ratio: f32,
    pub increase_restoration: f32,
    pub cleanse_dot_layers: u32,
}

impl Default for LunarRecovery {
    fn default() -> Self {
        Self {
            cooldown: 4,
            restore_hp_max_ratio: 0.16,
            increase_turn_meter_ratio: 0.2,
            increase_attack_chance: 1.0,
            increase_attack_turns: 2,
            increase_restoration: 0.4,
            increase_restoration_hp_ratio: 0.5,
            cleanse_dot_layers: 5,
        }
    }
}

impl LunarRecovery {
    pub const TYPE: SkillType = SkillType::Active;
    pub const SELECT: Select = Select::AllAllies;

    pub fn execute(
        &self,
        wave: &mut Wave,
        skill: &Skill,
        actor: InstanceIndex,
        defender: InstanceIndex,
    ) {
        wave.restore_max_hp_ratio_own_team(actor, self.restore_hp_max_ratio);
        wave.increase_turn_meter_team(actor, self.increase_turn_meter_ratio);
        wave.inflict_ally_team(
            actor,
            Effect::AttackUpI,
            self.increase_attack_chance,
            self.increase_attack_turns,
        );
        for i in wave.get_ally_indices(actor) {
            let fac =
                if wave.health[i] < wave.get_max_health(i) * self.increase_restoration_hp_ratio {
                    1.0 + self.increase_restoration
                } else {
                    1.0
                };
            wave.restore_single(
                actor,
                i,
                wave.get_max_health(i) * self.restore_hp_max_ratio * fac,
            );
        }
        wave.cleanse_team(actor, Effect::is_dot, self.cleanse_dot_layers);
    }
}
