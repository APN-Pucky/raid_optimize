

use crate::wave::heroes::{Cooldown, Skilled, Typed, Selector, Execute};
use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill, SkillType, Select}, effect::{Effect}, }, };

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct ScarletMultiStrike {
    pub cooldown : u32,
    pub attack_damage_ratio : f32,
}

impl Default for ScarletMultiStrike {
    fn default() -> Self {
        Self {
            cooldown : 4,
            attack_damage_ratio : 1.4
        }
    }
}

impl ScarletMultiStrike {
    pub const TYPE : SkillType = SkillType::Active;
    pub const SELECT : Select = Select::SingleEnemy;

    pub fn execute(&self, wave : &mut Wave,  skill : &Skill, attacker:InstanceIndex, defender:InstanceIndex, ) {
        wave.attack_single(attacker, defender,  wave.get_attack_damage(attacker)  *self.attack_damage_ratio, skill);
        wave.attack_single(attacker, defender,  wave.get_attack_damage(attacker)  *self.attack_damage_ratio, skill);
        wave.attack_single(attacker, defender,  wave.get_attack_damage(attacker)  *self.attack_damage_ratio, skill);
        wave.attack_single(attacker, defender,  wave.get_attack_damage(attacker)  *self.attack_damage_ratio, skill);
        wave.attack_single(attacker, defender,  wave.get_attack_damage(attacker)  *self.attack_damage_ratio, skill);
    }
}
