use crate::data::effect::Effect;
use crate::wave::heroes::{BasicAttack, Cooldown};
use crate::{
    data::skill::{Select, Skill, SkillType},
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct HolyArbitrament {
    pub cooldown: u32,
    pub defense_damage_ratio: f32,
    pub self_shield_damage_ratio: f32,
    pub ally_shield_damage_ratio: f32,
}

impl Default for HolyArbitrament {
    fn default() -> Self {
        Self {
            cooldown: 0,
            defense_damage_ratio: 3.8,
            self_shield_damage_ratio: 0.3,
            ally_shield_damage_ratio: 0.3,
        }
    }
}

impl HolyArbitrament {
    pub const TYPE: SkillType = SkillType::Basic;
    pub const SELECT: Select = Select::SingleEnemy;

    pub fn execute(
        &self,
        wave: &mut Wave,
        skill: &Skill,
        actor: InstanceIndex,
        defender: InstanceIndex,
    ) {
        wave.attack_single(
            actor,
            defender,
            self.defense_damage_ratio * wave.get_defense(actor),
            skill,
        );

        if wave.has_effect(actor, Effect::DivineLight) {
            let mut dmg = 0.;
            for i in wave.get_ally_indices(actor) {
                dmg += self.ally_shield_damage_ratio * wave.get_shield(i);
            }
            dmg += self.self_shield_damage_ratio * wave.get_shield(actor);
            wave.attack_single(actor, defender, dmg, skill);
        }
    }
}
