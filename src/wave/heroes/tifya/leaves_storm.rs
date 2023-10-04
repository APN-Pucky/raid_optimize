use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill}, effect::{Effect}, }, };

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct LeavesStorm {
    attack_damage_ratio : f32,
}

impl Default for LeavesStorm{
    fn default() -> Self {
        Self {
            attack_damage_ratio : 1.85
        }
    }
}

impl LeavesStorm {
    pub fn execute(&self, wave : &mut Wave,  skill : &Skill, attacker:InstanceIndex, defender:InstanceIndex, ) {
       wave.attack_enemy_team(attacker,   wave.get_attack_damage(attacker)  *self.attack_damage_ratio * (1. + 0.02 *wave.effects[attacker].get(Effect::ScarletSakura) as f32) , skill);
    }
}
