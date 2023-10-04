use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill}, effect::{Effect}, }, };

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct DarknightStrike{
        pub attack_damage_ratio : f32,
}

impl Default for DarknightStrike{
    fn default() -> Self {
        Self {
            attack_damage_ratio :0.8 
        }
    }
}

impl DarknightStrike{
    pub fn execute(&self, wave : &mut Wave,  skill : &Skill, attacker:InstanceIndex, defender:InstanceIndex, ) {
        wave.attack_single(attacker,defender, wave.get_attack_damage(attacker) * self.attack_damage_ratio, skill);
        wave.attack_single(attacker,defender, wave.get_attack_damage(attacker) * self.attack_damage_ratio, skill);
    }
}
