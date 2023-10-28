use strum::IntoEnumIterator;

use crate::data::effect::Effect;
use crate::wave::heroes::{BasicAttack, Cooldown};
use crate::{
    data::skill::{Select, Skill, SkillType},
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct HolyShelter {
    pub cooldown: u32,
    pub damage_imunity_turns: u32,
    pub block_debuff_turns: u32,
}

impl Default for HolyShelter {
    fn default() -> Self {
        Self {
            cooldown: 4,
            damage_imunity_turns: 2,
            block_debuff_turns: 2,
        }
    }
}

impl HolyShelter {
    pub const TYPE: SkillType = SkillType::Active;
    pub const SELECT: Select = Select::AllAllies;

    pub fn execute(
        &self,
        wave: &mut Wave,
        skill: &Skill,
        actor: InstanceIndex,
        defender: InstanceIndex,
    ) {
        wave.inflict_ally_team(actor, Effect::DamageImmunity, 1.0, 2);
        wave.inflict_ally_team(actor, Effect::BlockDebuff, 1.0, 2);

        // share buffs with allies
        for e in Effect::iter().filter(Effect::is_buff) {
            let v = wave.effects[actor].clone_single(e);
            if !v.is_empty() {
                for i in wave.get_ally_indices(actor) {
                    if i != actor {
                        wave.effects[i].extend_single(e, &v);
                    }
                }
            }
        }
    }
}
