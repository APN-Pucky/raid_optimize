use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill, SkillData}, effect::{Effect, is_dot}, }, };

pub mod scorched_soul;
pub mod fire_heal;
pub mod resurrection;


impl Wave<'_> {
    pub fn execute_skill_liz(&mut self,  skill : &Skill, actor :InstanceIndex, target :InstanceIndex, ) {
        let attacker = actor;
        let defender = target;
        match skill.data {
            SkillData::Resurrection(s)  => {
                s.execute(self,skill,attacker,defender);

            },
            SkillData::FireHeal(s) => {
                s.execute(self,skill,attacker,defender);
            },
            SkillData::ScorchedSoul(s) => {
                s.execute(self,skill,attacker,defender);
                //self.inflict_hp_burning(attacker,defender, hp_burning_chance, hp_burning_turns);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests;