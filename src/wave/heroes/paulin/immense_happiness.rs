use crate::data::effect::Effect;
use crate::roll;
use crate::wave::heroes::{BasicAttack, Cooldown};
use crate::{
    data::skill::{Select, Skill, SkillType},
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct ImmenseHappiness {
    pub cooldown: u32,
    pub hp_up_turns: u32,
    pub block_debuff_turns: u32,
    pub stealth_turns: u32,
    pub restore_hp_ratio: f32,
    pub act_again: bool,
}

impl Default for ImmenseHappiness {
    fn default() -> Self {
        Self {
            cooldown: 4,
            hp_up_turns: 2,
            block_debuff_turns: 1,
            restore_hp_ratio: 0.16,
            stealth_turns: 2,
            act_again: true,
        }
    }
}

impl ImmenseHappiness {
    pub const TYPE: SkillType = SkillType::Active;
    pub const SELECT: Select = Select::AllAllies;

    pub fn execute(
        &self,
        wave: &mut Wave,
        skill: &Skill,
        actor: InstanceIndex,
        defender: InstanceIndex,
    ) {
        if wave.get_effect_resistance(actor) >= 1.0 {
            wave.remove_effect_filter_allies(actor, Effect::is_debuff);
        }
        wave.inflict_ally_team(actor, Effect::HPUpII, 1.0, self.hp_up_turns);
        wave.restore_ally_team(actor, self.restore_hp_ratio * wave.get_max_health(actor));
        wave.inflict_ally_team(actor, Effect::Stealth, 1.0, self.stealth_turns);
        if wave.get_effect_resistance(actor) >= 1.0 {
            wave.inflict_ally_team(actor, Effect::BlockDebuff, 1.0, self.block_debuff_turns);
        }
        if self.act_again {
            wave.act(actor);
        }
    }
}
