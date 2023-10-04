use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill}, effect::{Effect}, }, };

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct BurstingKnowledge{
        pub attack_damage_ratio : f32,
        pub wisdom_runestones : u32,
        pub piercing_rate: f32,
}

impl Default for BurstingKnowledge{
    fn default() -> Self {
        Self {
            attack_damage_ratio : 1.2,
            wisdom_runestones : 4,
            piercing_rate: 0.,
        }
    }
}

impl BurstingKnowledge{
    pub fn execute(&self, wave : &mut Wave,  skill : &Skill, attacker:InstanceIndex, target:InstanceIndex, ) {
           // counter number of effects arcane
           let mut n = self.wisdom_runestones + wave.effects[attacker].get(Effect::Arcane);

           while  n > 0 {
               if wave.is_alive(target) {
                   wave.attack_single(attacker,target, wave.get_attack_damage(attacker) * self.attack_damage_ratio, skill);
               }
               else {
                   // get lowest hp enemy
                   let ene = wave.get_enemies_indices(attacker);
                   let mut lowest = ene[0];
                   for e in ene {
                       if wave.is_alive(e) && wave.health[e] < wave.health[lowest] {
                           lowest = e;
                       }
                   }
                   wave.attack_single(attacker,lowest, wave.get_attack_damage(attacker) * self.attack_damage_ratio, skill);
               }
               n = n-1;
           }
           // clear arcane
           wave.effects[target].clear_single(Effect::Arcane);
    }
}
