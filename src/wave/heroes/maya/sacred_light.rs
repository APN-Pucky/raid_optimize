use crate::wave::heroes::Cooldown;
use crate::{
    data::{
        effect::Effect,
        skill::{Select, Skill, SkillType},
    },
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct SacredLight {
    pub cooldown: u32,
    pub max_hp_restore_ratio: f32,
    pub loose_hp_ratio: f32,
    pub consolidation_turns: u32,
    pub shield_turns: u32,
    pub shield_max_hp_ratio: f32,
    pub block_debuff_turns: u32,
}

impl Default for SacredLight {
    fn default() -> Self {
        Self {
            cooldown: 3,
            max_hp_restore_ratio: 0.095,
            loose_hp_ratio: 0.35,
            consolidation_turns: 2,
            shield_turns: 2,
            shield_max_hp_ratio: 0.15,
            block_debuff_turns: 2,
        }
    }
}

impl SacredLight {
    pub const TYPE: SkillType = SkillType::Active;
    pub const SELECT: Select = Select::AllAllies;

    pub fn execute(
        &self,
        wave: &mut Wave,
        _skill: &Skill,
        actor: InstanceIndex,
        _target: InstanceIndex,
    ) {
        wave.loose_health(actor, self.loose_hp_ratio * wave.get_max_health(actor));
        // 1st
        for t in wave.get_ally_indices(actor) {
            if wave.health[t] == wave.get_max_health(t) {
                wave.inflict_single(
                    actor,
                    t,
                    Effect::ConsolidationI,
                    1.0,
                    self.consolidation_turns,
                );
            }
            wave.restore_single(
                actor,
                t,
                self.max_hp_restore_ratio * wave.get_max_health(actor),
            );
        }
        // 2nd
        for t in wave.get_ally_indices(actor) {
            if wave.health[t] == wave.get_max_health(t) {
                wave.shield_single(
                    actor,
                    t,
                    wave.get_max_health(actor) * self.shield_max_hp_ratio,
                    self.shield_turns,
                );
            }
            wave.restore_single(
                actor,
                t,
                self.max_hp_restore_ratio * wave.get_max_health(actor),
            );
        }
        // 3rd
        for t in wave.get_ally_indices(actor) {
            if wave.health[t] == wave.get_max_health(t) {
                wave.inflict_single(actor, t, Effect::BlockDebuff, 1.0, self.block_debuff_turns);
            }
            wave.restore_single(
                actor,
                t,
                self.max_hp_restore_ratio * wave.get_max_health(actor),
            );
        }
    }
}
