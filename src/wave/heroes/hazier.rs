use crate::{wave::{Wave, InstanceIndex}, data::{skill::Skill, effect::{Effect, is_buff}}, indent, debug};


impl<'a,const LEN:usize> Wave<'a,LEN> {
    pub fn execute_skill_hazier(&mut self,  skill : &Skill, actor :InstanceIndex, target :InstanceIndex, ) {
        let attacker = actor;
        let defender = target;
        match skill {
            Skill::DarknightStrike {basic_attack, attack_damage_ratio,.. }  => {
                self.attack_single(attacker,defender, (self.get_attack_damage(attacker) * attack_damage_ratio), skill);
                self.attack_single(attacker,defender, (self.get_attack_damage(attacker) * attack_damage_ratio), skill);
            },
            Skill::EyeForAnEye {counter_attack_turns,damage_immunity_turns,control_immunity_turns,..} => {
                self.inflict_single(actor,actor,Effect::CounterAttack,1.0,*counter_attack_turns);
                self.inflict_single(actor,actor,Effect::DamageImmunity,1.0,*damage_immunity_turns);
                self.inflict_single(actor,actor,Effect::ControlImmunity,1.0,*control_immunity_turns);
                self.act(actor);
                //self.inflict_single(actor,actor,Effect::CounterAttack,1.0,3);
            },
            Skill::DarknightArbitrament {attack_damage_ratio,..} => {
                self.inflict_single(actor,actor,Effect::CritRateUpI,1.0,2);
                self.inflict_single(actor,actor,Effect::CritDamageUpI,1.0,2);
                // copy buffs from defender to self
                debug!("Copying buffs from {:?} to {:?}",self.name(defender),self.name(actor));
                indent!({
                    for (effect,vec) in self.effects[defender].em.iter() {
                        if is_buff(effect) {
                            self.effects[actor].em[effect].extend(vec.iter().cloned());
                        }
                    }
                });
                self.attack_single(attacker,defender, (self.get_attack_damage(attacker) * attack_damage_ratio), skill);

            },
            _ => {}

        }
    }
}