use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill, SkillData}, effect::{Effect, is_buff}}, indent, debug};


impl<'a,const LEN:usize> Wave<'a,LEN> {
    pub fn execute_skill_hazier(&mut self,  skill : &Skill, actor :InstanceIndex, target :InstanceIndex, ) {
        let attacker = actor;
        let defender = target;
        match skill.data {
            SkillData::DarknightStrike { attack_damage_ratio,.. }  => {
                self.attack_single(attacker,defender, (self.get_attack_damage(attacker) * attack_damage_ratio), skill);
                self.attack_single(attacker,defender, (self.get_attack_damage(attacker) * attack_damage_ratio), skill);
            },
            SkillData::EyeForAnEye {counterattack_turns: counter_attack_turns,damage_immunity_turns,control_immunity_turns,..} => {
                self.inflict_single(actor,actor,Effect::CounterAttack,1.0,counter_attack_turns);
                self.inflict_single(actor,actor,Effect::DamageImmunity,1.0,damage_immunity_turns);
                self.inflict_single(actor,actor,Effect::ControlImmunity,1.0,control_immunity_turns);
                self.act(actor);
                //self.inflict_single(actor,actor,Effect::CounterAttack,1.0,3);
            },
            SkillData::DarknightArbitrament {crit_rate_turns, crit_damage_turns , attack_damage_ratio,..} => {
                self.inflict_single(actor,actor,Effect::CritRateUpI,1.0,crit_rate_turns);
                self.inflict_single(actor,actor,Effect::CritDamageUpI,1.0,crit_damage_turns);
                // copy buffs from defender to self
                debug!("Copying buffs from {:?} to {:?}",self.name(defender),self.name(actor));
                indent!({
                    //cloned loop to allow copying of buffs from self to self, which should never happen
                    for effect in self.effects[defender].em.iter().map(|(effect,_)| effect).filter(|effect| is_buff(*effect)).collect::<Vec<_>>() {
                        let cloned_vec: Vec<_> = self.effects[defender].em[effect].iter().cloned().collect();
                        self.effects[actor].em[effect].extend(cloned_vec);
                    }
                });
                self.attack_single(attacker,defender, (self.get_attack_damage(attacker) * attack_damage_ratio), skill);

            },
            _ => {}

        }
    }
}