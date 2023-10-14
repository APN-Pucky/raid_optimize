use crate::{data::{skill::{Skill, get_cooldown, is_basic_attack, is_passive, is_reducable, Generic}, faction::Faction, effect::Effect}, indent, debug};

use super::{InstanceIndex, Wave, stat::Stat};

pub mod subskills;


pub type SkillIndex = usize;
impl<'a> Wave<'a> {

    pub fn get_active_skills(&self, actor: InstanceIndex) -> Vec<&'a Skill> {
        self.heroes[actor].skills.iter()
            .zip(self.cooldowns[actor].iter())
            .filter_map(|(s,c)| if *c == 0 && !is_passive(s) {Some(s)} else {None})
            .collect()
    }
}

impl Wave<'_> {
    pub fn pre_execute_skill(&mut self, actor: InstanceIndex,_target: InstanceIndex, skill: &Skill) {
        debug!("{} pre_execute_skill {}", self.name(actor), skill);
        indent!({
            if self.get_faction(actor) == Faction::HiddenWave {
                if is_basic_attack(skill) {
                    self.inflict_single(actor, actor, Effect::FactionHiddenWaveAttack,1.0, 2);
                }
                else {
                    self.inflict_single(actor, actor, Effect::FactionHiddenWaveSkill,1.0, 2);
                }
            }

        })
    }

    pub fn cooldown_s(&mut self,actor: InstanceIndex, skill:&Skill) {
        if let Some(i) = self.heroes[actor].skills.iter().position(|s| s == skill) {
            self.cooldowns[actor][i] = get_cooldown(skill);
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

    pub fn reset_skill(&mut self, actor: InstanceIndex,target : InstanceIndex, skill_index: SkillIndex) {
        self.add_stat(actor,Stat::ResetSkill , 1.);
        if is_reducable(self.get_skill(target,skill_index)) {
            self.cooldowns[target][skill_index] = 0;
        }
    }

    pub fn reset_all_skills(&mut self, actor: InstanceIndex,target : InstanceIndex ) {
        for si in 0..self.cooldowns[target].len() {
            self.reset_skill(actor, target, si);
        }
    }

    
    pub fn turn_reduce_cooldowns(&mut self,actor: InstanceIndex) {
        debug!("Reducing cooldowns for {} ({}):", self.name(actor), self.cooldowns[actor].len());
        self.cooldowns[actor].iter_mut().for_each(|c| *c = c.saturating_sub(1));
        indent!({
            for (i,c) in self.cooldowns[actor].iter().enumerate() {
                debug!("{}: {}", self.get_skill(actor,i), c);
            }
        })
    }

    pub fn reduce_cooldowns(&mut self,actor: InstanceIndex) {
        debug!("Reducing cooldowns for {} ({}):", self.name(actor), self.cooldowns[actor].len());
        self.cooldowns[actor].iter_mut().enumerate().filter(|(i,_)| is_reducable(&self.heroes[actor].skills[*i as SkillIndex] )).for_each(|(_,c)| *c = c.saturating_sub(1));
        indent!({
            for (i,c) in self.cooldowns[actor].iter().enumerate() {
                debug!("{}: {}", self.get_skill(actor,i), c);
            }
        })
    }
    
    pub fn reset_all_cooldowns(&mut self,actor: InstanceIndex) {
        debug!("Resetting cooldowns for {} ({}):", self.name(actor), self.cooldowns[actor].len());
        self.cooldowns[actor].iter_mut().for_each(|c| *c = 0);
    }

    pub fn cooldown(&mut self, actor: InstanceIndex,skill : SkillIndex) {
        self.cooldowns[actor][skill] = get_cooldown(&self.get_hero(actor).skills[skill]);
    }

    pub fn is_ready(&mut self, actor: InstanceIndex,skill:SkillIndex) -> bool{
        self.cooldowns[actor][skill] == 0
    }

    pub fn execute_generic_skill(&mut self, skill : &Skill, actor :InstanceIndex, target :InstanceIndex, ) {
        if let Skill::Generic(Generic{ subskills ,..}) = skill {
            for ss in subskills {
                self.execute_subskill(&ss, actor, Some(target),skill);
            }
        }
    }

   
    //}
}