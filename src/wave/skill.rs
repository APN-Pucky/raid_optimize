use crate::hero::skill::{Skill, get_cooldown};

use super::{InstanceIndex, Wave};



pub type SkillIndex = usize;
impl<'a,const LEN:usize> Wave<'a,LEN> {

    pub fn get_active_skills(&self, actor: InstanceIndex) -> Vec<&'a Skill> {
        self.heroes[actor].skills.iter()
            .zip(self.cooldowns[actor].iter())
            .filter_map(|(s,c)| if *c == 0 {Some(s)} else {None})
            .collect()
    }
}

impl<const LEN:usize> Wave<'_,LEN> {

    pub fn cooldown_s(&mut self,actor: InstanceIndex, skill:&Skill) {
        if let Some(i) = self.heroes[actor].skills.iter().position(|s| s == skill) {
            self.cooldowns[actor][i] = *get_cooldown(skill);
        }
        else {
            panic!("Skill {:?} not found in hero {:?}", skill, self.heroes[actor]);
        }
 
    }

    pub fn get_skill_index(&self,actor: InstanceIndex, skill: &Skill) -> SkillIndex{
        self.get_hero(actor).skills.iter().position(|s| s == skill).unwrap()
    }

    pub fn get_skill(&self,actor: InstanceIndex, skill_index: SkillIndex) -> &Skill {
        &self.get_hero(actor).skills[skill_index]
    }

    
    pub fn reduce_cooldowns(&mut self,actor: InstanceIndex) {
        self.cooldowns[actor].iter_mut().for_each(|c| *c = c.saturating_sub(1));
    }

    pub fn cooldown(&mut self, actor: InstanceIndex,skill : SkillIndex) {
        self.cooldowns[actor][skill] = *get_cooldown(&self.get_hero(actor).skills[skill]);
    }
}