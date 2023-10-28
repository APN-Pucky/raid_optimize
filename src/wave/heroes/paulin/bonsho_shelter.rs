use crate::data::effect::Effect;
use crate::roll;
use crate::wave::heroes::{BasicAttack, Cooldown};
use crate::{
    data::skill::{Select, Skill, SkillType},
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct BonshoShelter {
    pub cooldown: u32,
    pub damage_immunity_turns: u32,
    pub defense_up_turns: u32,
    pub tenacity_up_turns: u32,
}

impl Default for BonshoShelter {
    fn default() -> Self {
        Self {
            cooldown: 3,
            damage_immunity_turns: 1,
            tenacity_up_turns: 2,
            defense_up_turns: 2,
        }
    }
}

impl BonshoShelter {
    pub const TYPE: SkillType = SkillType::Active;
    pub const SELECT: Select = Select::AllAllies;

    pub fn execute(
        &self,
        wave: &mut Wave,
        skill: &Skill,
        actor: InstanceIndex,
        defender: InstanceIndex,
    ) {
        wave.inflict_ally_team(actor, Effect::TenacityUpII, 1.0, self.tenacity_up_turns);
        wave.inflict_ally_team(actor, Effect::DefenseUpII, 1.0, self.defense_up_turns);

        if wave.get_effect_resistance(actor) >= 1.0 {
            wave.inflict_single(
                actor,
                actor,
                Effect::DamageImmunity,
                1.0,
                self.damage_immunity_turns,
            );
        }
    }
}
