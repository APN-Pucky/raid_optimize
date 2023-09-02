use crate::{wave::{Wave, InstanceIndex}, data::{skill::Skill, effect::Effect}};


impl<'a,const LEN:usize> Wave<'a,LEN> {
    pub fn execute_skill_space(&mut self,  skill : &Skill, actor :InstanceIndex, target :InstanceIndex, ) {
        let attacker = actor;
        let defender = target;
        match skill {
            Skill::DarknightStrike {basic_attack, attack_damage_ratio,.. }  => {
                self.attack_single(attacker,defender, (self.get_attack_damage(attacker) * attack_damage_ratio), skill);
                self.attack_single(attacker,defender, (self.get_attack_damage(attacker) * attack_damage_ratio), skill);
            },
            Skill::EyeForAnEye {} => {
            },
            Skill::DarknightArbitrament {} => {

            },
            _ => {}

        }
    }
}