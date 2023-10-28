use crate::data::effect::Effect;
use crate::roll;
use crate::wave::heroes::{BasicAttack, Cooldown};
use crate::{
    data::skill::{Select, Skill, SkillType},
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct RegalMight {
    pub cooldown: u32,
    pub attack_damage_ratio: f32,
    pub increase_active_cooldowns_chance: f32,
    pub increase_active_cooldowns_turns: u32,
    pub steal_turn_meter_ratio: f32,
}

impl Default for RegalMight {
    fn default() -> Self {
        Self {
            cooldown: 3,
            attack_damage_ratio: 1.0,
            increase_active_cooldowns_chance: 0.5,
            increase_active_cooldowns_turns: 1,
            steal_turn_meter_ratio: 0.10,
        }
    }
}

impl RegalMight {
    pub const TYPE: SkillType = SkillType::Active;
    pub const SELECT: Select = Select::AllEnemies;

    pub fn execute(
        &self,
        wave: &mut Wave,
        skill: &Skill,
        actor: InstanceIndex,
        defender: InstanceIndex,
    ) {
        wave.attack_enemy_team(
            actor,
            self.attack_damage_ratio * wave.get_attack(actor),
            skill,
        );
        wave.attack_enemy_team(
            actor,
            self.attack_damage_ratio * wave.get_attack(actor),
            skill,
        );

        for t in wave.get_enemies_indices(actor) {
            if roll(self.increase_active_cooldowns_chance) {
                if wave.increase_all_cooldowns(actor, t, self.increase_active_cooldowns_turns) > 0 {
                    wave.steal_turn_meter_ratio(actor, t, self.steal_turn_meter_ratio);
                }
            }
        }
    }
}
