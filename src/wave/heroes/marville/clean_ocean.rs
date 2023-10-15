use crate::data::effect::{is_attribute_debuff, is_dot};
use crate::wave::heroes::Cooldown;
use crate::{
    data::{
        effect::Effect,
        skill::{Select, Skill, SkillType},
    },
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct CleanOcean {
    pub cooldown: u32,
    pub restore_max_hp_ratio: f32,
    pub cleanse_dot_layers: u32,
    pub consolidation_turns: u32,
    pub block_removal_turns: u32,
}

impl Default for CleanOcean {
    fn default() -> Self {
        Self {
            cooldown: 4,
            restore_max_hp_ratio: 0.14,
            cleanse_dot_layers: 5,
            consolidation_turns: 2,
            block_removal_turns: 2,
        }
    }
}

impl CleanOcean {
    pub const TYPE: SkillType = SkillType::Active;
    pub const SELECT: Select = Select::AllAllies;

    pub fn execute(
        &self,
        wave: &mut Wave,
        _skill: &Skill,
        actor: InstanceIndex,
        _target: InstanceIndex,
    ) {
        wave.restore_max_hp_ratio_own_team(actor, self.restore_max_hp_ratio);
        wave.cleanse_team(actor, &is_dot, self.cleanse_dot_layers);
        wave.cleanse_team(actor, &is_attribute_debuff, 999);
        wave.inflict_ally_team(actor, Effect::BlockRemoval, 1.0, self.block_removal_turns);
        wave.inflict_ally_team(
            actor,
            Effect::ConsolidationII,
            1.0,
            self.consolidation_turns,
        );
    }
}
