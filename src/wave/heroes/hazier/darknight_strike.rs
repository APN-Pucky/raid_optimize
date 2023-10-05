

use crate::wave::heroes::{Cooldown, Skilled, Typed, Selector, Execute};
use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill, SkillType, Select}, effect::{Effect}, }, };

#[derive(Cooldown,Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct DarknightStrike{
    pub cooldown : u32,
    pub attack_damage_ratio : f32,
}

impl Default for DarknightStrike{
    fn default() -> Self {
        Self {
            cooldown : 0,
            attack_damage_ratio :0.8 
        }
    }
}

impl DarknightStrike{
    pub const TYPE : SkillType = SkillType::Basic;
    pub const SELECT : Select = Select::AllEnemies;

    pub fn execute(&self, wave : &mut Wave,  skill : &Skill, attacker:InstanceIndex, defender:InstanceIndex, ) {
        wave.attack_single(attacker,defender, wave.get_attack_damage(attacker) * self.attack_damage_ratio, skill);
        wave.attack_single(attacker,defender, wave.get_attack_damage(attacker) * self.attack_damage_ratio, skill);
    }
}
