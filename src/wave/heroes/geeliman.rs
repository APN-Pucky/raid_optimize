use crate::{wave::{Wave, InstanceIndex}, data::{skill::Skill, effect::Effect}};


impl<'a,const LEN:usize> Wave<'a,LEN> {
    pub fn execute_skill_geeliman(&mut self,  skill : &Skill, actor :InstanceIndex, target :InstanceIndex, ) {
        let attacker = actor;
        let defender = target;
        match skill {
            Skill::BurstingKnowledge{ attack_damage_ratio, wisdom_runestones, cooldown, basic_attack, piercing_rate } => {
                // counter number of effects arcane
                let mut n = wisdom_runestones + self.effects[actor].get(Effect::Arcane);

                while  n > 0 {
                    if self.is_alive(target) {
                        self.attack_single(attacker,defender, (self.get_attack_damage(attacker) * attack_damage_ratio ), skill);
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
                        self.attack_single(attacker,lowest, (self.get_attack_damage(attacker) * attack_damage_ratio ), skill);
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