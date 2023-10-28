use crate::data::effect::Effect;
use crate::wave::heroes::{BasicAttack, Cooldown};
use crate::{
    data::skill::{Select, Skill, SkillType},
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct KingsMajesty {
    pub cooldown: u32,
    pub block_buff_chance: f32,
    pub block_buff_turns: u32,
    pub effect_resistance_turns: u32,
    pub reset_cooldown: bool,
}

impl Default for KingsMajesty {
    fn default() -> Self {
        Self {
            cooldown: 5,
            block_buff_chance: 1.0,
            block_buff_turns: 1,
            effect_resistance_turns: 2,
            reset_cooldown: true,
        }
    }
}

impl KingsMajesty {
    pub const TYPE: SkillType = SkillType::Active;
    pub const SELECT: Select = Select::SingleEnemy;

    pub fn execute(
        &self,
        wave: &mut Wave,
        skill: &Skill,
        actor: InstanceIndex,
        defender: InstanceIndex,
    ) {
        wave.remove_effect_filter_single(actor, defender, Effect::is_buff);
        wave.inflict_single(
            actor,
            defender,
            Effect::BlockBuff,
            self.block_buff_chance,
            self.block_buff_turns,
        );
        wave.remove_effect_filter_ally(actor, Effect::is_control);

        for high in wave.get_ally_indices(actor) {
            wave.attack_single(
                high,
                defender,
                wave.get_attack(high),
                &Skill::BasicAttack(BasicAttack::default()),
            );
        }
        if wave.is_alive(defender) && self.reset_cooldown {
            // reset cooldown
            wave.reset_skill(actor, actor, wave.get_skill_index(actor, skill));
        }
    }
}
