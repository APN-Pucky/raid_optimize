use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill}, effect::{Effect}, }, };

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct ScarletSlash{
    attack_damage_ratio : f32,
}

impl Default for ScarletSlash{
    fn default() -> Self {
        Self {
            attack_damage_ratio : 1.2
        }
    }
}

impl ScarletSlash{
    pub fn execute(&self, wave : &mut Wave,  skill : &Skill, attacker:InstanceIndex, defender:InstanceIndex, ) {
        wave.attack_single(attacker, defender,  wave.get_attack_damage(attacker)  *self.attack_damage_ratio, skill);
        wave.attack_single(attacker, defender,  wave.get_attack_damage(attacker)  *self.attack_damage_ratio, skill);
    }
}
