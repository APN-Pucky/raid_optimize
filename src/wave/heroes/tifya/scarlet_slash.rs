

use crate::wave::heroes::Cooldown;
use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill, SkillType, Select}, effect::{Effect}, }, };

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct ScarletSlash{
    pub cooldown : u32,
    pub attack_damage_ratio : f32,
}

impl Default for ScarletSlash{
    fn default() -> Self {
        Self {
            cooldown : 0,
            attack_damage_ratio : 1.2
        }
    }
}

impl ScarletSlash{
    pub const TYPE : SkillType = SkillType::Basic;
    pub const SELECT : Select = Select::SingleEnemy;

    pub fn execute(&self, wave : &mut Wave,  skill : &Skill, attacker:InstanceIndex, defender:InstanceIndex, ) {
        wave.attack_single(attacker, defender,  wave.get_attack_damage(attacker)  *self.attack_damage_ratio, skill);
        wave.attack_single(attacker, defender,  wave.get_attack_damage(attacker)  *self.attack_damage_ratio, skill);
    }
}