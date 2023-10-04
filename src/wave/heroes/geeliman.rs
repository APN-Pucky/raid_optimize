use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill, SkillData}, effect::Effect}};


impl Wave<'_> {
    pub fn execute_skill_geeliman(&mut self,  skill : &Skill, actor :InstanceIndex, target :InstanceIndex, ) {
        let attacker = actor;
        let defender = target;
        match skill.data {
            SkillData::BurstingKnowledge{ attack_damage_ratio, wisdom_runestones,   piercing_rate: _ } => {
                // counter number of effects arcane
                let mut n = wisdom_runestones + self.effects[actor].get(Effect::Arcane);

                while  n > 0 {
                    if self.is_alive(target) {
                        self.attack_single(attacker,defender, self.get_attack_damage(attacker) * attack_damage_ratio, skill);
                    }
                    else {
                        // get lowest hp enemy
                        let ene = self.get_enemies_indices(actor);
                        let mut lowest = ene[0];
                        for e in ene {
                            if self.is_alive(e) && self.health[e] < self.health[lowest] {
                                lowest = e;
                            }
                        }
                        self.attack_single(attacker,lowest, self.get_attack_damage(attacker) * attack_damage_ratio, skill);
                    }
                    n = n-1;
                }
                // clear arcane
                self.effects[target].clear_single(Effect::Arcane);
            },
            _ => {}

        }
    }
}