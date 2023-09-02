use crate::{data::{skill::{Skill, get_cooldown, is_basic_attack}, faction::Faction}, indent, debug};

use super::{InstanceIndex, Wave};

pub mod subskills;


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
    pub fn pre_execute_skill(&mut self, actor: InstanceIndex,target: InstanceIndex, skill: &Skill) {}
    /* FIXME 
    pub fn pre_execute_skill(&mut self, actor: InstanceIndex,target: InstanceIndex, skill: &Skill) {
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
    */

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

    
    pub fn reduce_cooldowns(&mut self,actor: InstanceIndex) {
        debug!("Reducing cooldowns for {} ({}):", self.name(actor), self.cooldowns[actor].len());
        self.cooldowns[actor].iter_mut().for_each(|c| *c = c.saturating_sub(1));
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

    pub fn execute_generic_skill(&mut self, skill : &Skill, actor :InstanceIndex, target :InstanceIndex, ) {
        if let Skill::Generic{ basic_attack,cooldown, subskills ,..} = skill {
            for ss in subskills {
                self.execute_subskill(ss, actor, target,skill);
            }
        }
    }

    pub fn execute_skill(&mut self,  skill : &Skill, actor :InstanceIndex, target :InstanceIndex, ) {
        }

        /* 
    pub fn execute_skill(&mut self,  skill : &Skill, actor :InstanceIndex, target :InstanceIndex, ) {
        // if let generic 
        execute_generic_skill(self, skill, actor, target);
        execute_skill_tifya(self, skill, actor, target);
        execute_skill_space(self, skill, actor, target);


        //let wave = self;
        let attacker = actor;
        let defender = target;


        match skill {
            Skill::Resurrection { shield_max_hp_ratio, shield_turns, cleanse_dot_debuffs, restore_max_hp_ratio ,..} => {
                let max_hp = self.get_max_health(actor);
                self.restore_max_hp_ratio_own_team(actor,*restore_max_hp_ratio);
                self.shield_ally_team(actor,shield_max_hp_ratio * max_hp  ,*shield_turns);
                //TODO
                //self.cleanse_ally_team(actor,&is_dot,*cleanse_dot_debuffs);
            },

            Skill::DeepSeaPower {  max_hp_shield_ratio, shield_turns, tenacity_increase_turns ,..} => {
                let max_hp = self.get_max_health(actor);
                self.shield_ally_team(actor,max_hp_shield_ratio * max_hp  ,*shield_turns);
                self.inflict_ally_team(actor, Effect::TenacityUpII, 1.0, *tenacity_increase_turns);
            },
            Skill::CrystalOfLife {  max_hp_restore_ratio, ripple_turns , attack_up_turns ,..} =>{
                let rest_hp = (self.get_max_health(actor)  * max_hp_restore_ratio) ;
                self.restore_ally_team(actor,rest_hp);
                self.inflict_ally_team(actor, Effect::RippleII, 1.0, *ripple_turns);
                self.inflict_ally_team(actor, Effect::AttackUpII, 1.0,* attack_up_turns);
            },

            Skill::FireHeal{heal_attack_ratio,heal_max_hp_ratio,block_debuff_turns,..} => {
                let heal = self.get_attack_damage(actor)*heal_attack_ratio ;
                let max_hp_heal = self.get_max_health(actor)*heal_max_hp_ratio ;
                self.restore(actor,target, heal + max_hp_heal);
                self.inflict_single(actor,target,Effect::BlockDebuf, 1.0,*block_debuff_turns);
            }
            Skill::ScorchedSoul{basic_attack,attack_damage_ratio,hp_burning_chance, hp_burning_turns ,..} => {
                self.attack_single(attacker, defender,  self.get_attack_damage(attacker)  *attack_damage_ratio, skill);
                self.inflict_single(attacker,defender,Effect::HPBurning, *hp_burning_chance, *hp_burning_turns);
                //self.inflict_hp_burning(attacker,defender, *hp_burning_chance, *hp_burning_turns);
            }

            Skill::BasicAttack{basic_attack,attack_damage_ratio,..} => {
                self.attack_single(attacker,defender, (self.get_attack_damage(attacker) * attack_damage_ratio), skill);
            }


            Skill::TideBigHit {basic_attack, max_hp_damage_ratio, suffocated_chance, suffocated_turns,.. } => {
                log::debug!("{} uses Tide Big Hit on {}", attacker, defender);
                let mut chance = *suffocated_chance;
                self.attack_single(attacker,defender, (self.get_max_health(attacker) * max_hp_damage_ratio), skill);
                if self.has_effect(defender,Effect::WetI) 
                || self.has_effect(defender,Effect::WetII) 
                || self.has_effect(defender,Effect::ColdI) 
                || self.has_effect(defender,Effect::ColdII){
                    log::debug!("{} is wet or cold +15% suffocation chance", defender);
                    chance += 0.15;
                }
                self.inflict_single(attacker,defender,Effect::Suffocated, chance,*suffocated_turns);
            },

            //_ => panic!("Skill not implemented"),
        }
        self.cooldown_s(attacker,skill);
    }
    */
}