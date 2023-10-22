use crate::wave::heroes::Cooldown;
use crate::{
    data::{
        effect::Effect,
        skill::{Select, Skill, SkillType},
    },
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct BloodySpiral {
    pub cooldown: u32,
    pub attack_damage_ratio: f32,
    pub increase_damage_ratio: f32,
    pub restore_hp_damage_ratio: f32,
}

impl Default for BloodySpiral {
    fn default() -> Self {
        Self {
            cooldown: 3,
            attack_damage_ratio: 7.0,
            increase_damage_ratio: 0.2,
            restore_hp_damage_ratio: 0.1,
        }
    }
}

impl BloodySpiral {
    pub const TYPE: SkillType = SkillType::Active;
    pub const SELECT: Select = Select::SingleEnemy;

    pub fn execute(
        &self,
        wave: &mut Wave,
        skill: &Skill,
        actor: InstanceIndex,
        defender: InstanceIndex,
    ) {
        let i = wave.remove_all_buffs_single(actor, defender);
        let dmg_inc = if i > 0 {
            1.0 + self.increase_damage_ratio
        } else {
            1.0
        };
        let dmg_dealt = wave.attack_single(
            actor,
            defender,
            self.attack_damage_ratio * wave.get_attack_damage(actor) * dmg_inc,
            skill,
        );
        wave.restore_ally_team(actor, self.restore_hp_damage_ratio * dmg_dealt);
    }
}
