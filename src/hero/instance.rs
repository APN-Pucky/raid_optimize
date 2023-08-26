// Import (via `use`) the `fmt` module to make it available.
use std::fmt;
use enum_map::EnumMap;

use rand::seq::SliceRandom;
use rand::Rng;


use crate::hero::Hero;
use crate::{roll, indent, debug, warn, info, error};
use crate::wave::{  InstanceIndex, TeamIndex};
use crate::hero::skill::Skill;
use crate::hero::effect::Effect;

use super::effects::Effects;
use super::passive::Passive;
use super::skill::get_cooldown;


//TODO make instance so irrelevant that it can be removed since it only wraps hero

#[derive(Debug)]
pub struct Instance<'a> {
    pub hero:  &'a Hero,
    pub id : u32,
    pub index: InstanceIndex,
    pub team : TeamIndex,
    // these are transported between waves
    pub health: f32,
    pub turn_meter: f32,
    pub cooldowns : Vec<u32>,
}

impl fmt::Display for Instance<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{} [health: {}, turn_meter: {}]", self.hero.name, self.id,self.health, self.turn_meter)
    }
}

impl PartialEq for Instance<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'a> Instance<'a> {

    pub fn get_active_skills(&self) -> Vec<&'a Skill> {
        self.hero.skills.iter()
            .zip(self.cooldowns.iter())
            .filter_map(|(s,c)| if *c == 0 {Some(s)} else {None})
            .collect()
        //let mut ret  = Vec::new();
        //for i in 0..self.cooldowns.len() {
        //    if self.cooldowns[i] == 0 {
        //        ret.push(self.hero.skills[i]);
        //    }
        //}
        //ret
    }
}

impl Instance<'_> {
    pub fn new(hero: &Hero, id:u32,ii:InstanceIndex,ti:TeamIndex) -> Instance {
        Instance {
            hero: hero,
            id,
            index:ii,
            team:ti,
            health: hero.health,
            turn_meter: 0.0,
            cooldowns : hero.skills.iter().map(|_| 0).collect(),
            //shield: Vec::new(),
            //statistics: EnumMap::default(),
            //effects : Effects::new(), 
            //passives : hero.passives.clone(),
        }
    }

    pub fn has_passive(&self, passive: Passive) -> bool {
        self.hero.passives.iter().any(|p| *p == passive)
    }

    pub fn cooldown(&mut self, skill :usize) {
        self.cooldowns[skill] = *get_cooldown(&self.hero.skills[skill]);//*get_cooldown(skill);
        // find index of skill in hero.skills
        //if let Some(i) = self.hero.skills.iter().position(|s| s == skill) {
        //    self.cooldowns[i] = *get_cooldown(skill);
        //}
        //else {
        //    panic!("Skill {:?} not found in hero {:?}", skill, self.hero);
        //}
    }

    pub fn cooldown_s(&mut self, skill:&Skill) {
        if let Some(i) = self.hero.skills.iter().position(|s| s == skill) {
            self.cooldowns[i] = *get_cooldown(skill);
        }
        else {
            panic!("Skill {:?} not found in hero {:?}", skill, self.hero);
        }
 
    }
    
    pub fn reduce_cooldowns(&mut self) {
        self.cooldowns.iter_mut().for_each(|c| *c = c.saturating_sub(1));
        //for i in 0..self.cooldowns.len() {
        //    if self.cooldowns[i] > 0 {
        //        self.cooldowns[i] -= 1;
        //    }
        //}
    }

    #[deprecated]
    pub fn get_cooldown_mask(&self) -> Vec<bool> {
        self.cooldowns.iter().map(|c| *c == 0).collect()
    }

    pub fn get_skill_ref(&self, skill: &Skill) -> usize {
        self.hero.skills.iter().position(|s| s == skill).unwrap()
    }

    pub fn get_skill(&self, skill_ref: usize) -> &Skill {
        &self.hero.skills[skill_ref]
    }

    pub fn is_dead(&self) -> bool {
        !self.is_alive()
    }
    /*

    pub fn heal(&mut self, health: f32) {
        if self.is_dead() {
            warn!("{} is dead, cannot heal [{},{}]", self,self.health,self.health > 0.0);
            return;
        }
        let heal = health * (1.+self.hero.healing_effect); // TODO handle rounding
        let new_health = self.hero.health.min(self.health + heal);
        debug!("{} heals {} health (healing_effect: {})", self, heal, self.hero.healing_effect);
        self.add_stat(Stat::HealthHealed, new_health- self.health );
        self.health = new_health;
    }

    //pub fn copy_statistics(&self) -> EnumMap<Stat, u32> {
    //    self.statistics.clone()
    //}


    */
    //pub fn reduce_turn_meter(&mut self, target:&mut Instance, turn_meter_reduction_ratio: f32) {
    //    // relative to total!
    //    let mut red = (TURN_METER_THRESHOLD * turn_meter_reduction_ratio);
    //    let turn = target.get_turn_meter();
    //    if red > turn {
    //        red  = turn
    //    }
    //    let turn_meter = turn - red;
    //    debug!("{} reduces turn meter of {} by {} to {}", self, target, turn_meter_reduction_ratio, turn_meter);
    //    target.set_turn_meter(turn_meter);
    //    //self.add_stat(Stat::TurnMeterReduced, turn_meter);
    //}

    /*
    pub fn take_hp_burning_damage(&mut self, dmg: f32) {
        debug!("{} takes {} damage from hp_burning", self, dmg);
        // todo handle mastery
        self.take_damage(dmg);
    }

    pub fn take_bleed_damage(&mut self, bleed_dmg: f32) {
        debug!("{} takes {} damage from bleed", self, bleed_dmg);
        // todo handle mastery
        self.take_damage(bleed_dmg);
    }


    pub fn loose_health(&mut self, damage: f32) {
        debug!("{} looses {} health", self, damage);
        if self.health < damage {
            self.add_stat(Stat::HealthLost, self.health);
            self.health = 0.0;
        }
        else {
            self.add_stat(Stat::HealthLost, damage);
            self.health -= damage;
        }
    }

    pub fn take_damage(&mut self, damage: f32) {
        debug!("{} takes {} damage", self, damage);
        self.add_stat(Stat::DamageTaken, damage);
        let dmg = self.loose_shield(damage);
        self.loose_health(dmg);
    }

    pub fn deal_damage(&mut self, target: &mut Instance, damage: f32) {
        debug!("{} takes {} damage from {}", target , damage , self);
        self.add_stat(Stat::DamageDone, damage);
        target.take_damage(damage);
        let leech = (damage * self.hero.leech) ; // TODO handle rounding
        if leech > 0.0 {
            self.add_stat(Stat::Leeched, leech);
            self.heal(leech);
        }
    }

    pub fn attack(&mut self, target: &mut Instance, atk_dmg:f32 ) {
        // test if critical strike
        debug!("{} attacks {} with {} attack", self, target,atk_dmg);
        indent!({
        self.add_stat(Stat::Attacks, 1.0);
        let mut rng = rand::thread_rng();
        let crit = rng.gen::<f32>() < self.hero.crit_rate;
        let mut attack  = atk_dmg; //(self.get_attack() as f32 * dmg_ratio) as u32;
        if crit {
            self.add_stat(Stat::CriticalStrikes, 1.0);
            let crit = self.hero.crit_damage;
            let mut tenacity = target.hero.tenacity;
            if tenacity > crit {
                tenacity = crit;
            }
            let crit_rate = crit - tenacity;
            self.add_stat(Stat::CriticalDamage, attack  * crit_rate  );
            target.add_stat(Stat::TenacityIgnored, attack  * tenacity );
            attack = (attack * crit_rate);
            debug!("{} critical attack ({}%={}%-{}%)", self,crit_rate*100.,crit*100.,tenacity*100.);
        }
        self.add_stat(Stat::Attack, attack);
        let mut def = target.get_defense();

        let pierce = (def  * self.hero.piercing); // TODO handle rounding
        self.add_stat(Stat::PiercedDefense, pierce);
        debug!("{} pierces {} defense of {} ({}%)", self, pierce, def, self.hero.piercing*100.);
        def -= pierce;
        
        self.deal_damage(target, attack - def);
        })
    }
    */

    pub fn reset_turn_meter(&mut self) {
        self.set_turn_meter(0.0)
    }

    pub fn set_turn_meter(&mut self, turn_meter: f32) { indent!({
        debug!("{} turn_meter set to {}", self, turn_meter);
        self.turn_meter= turn_meter
    })}

    pub fn increase_turn_meter(&mut self, turn_meter: f32) {
        self.turn_meter+= turn_meter
    }

    pub fn decrease_turn_meter(&mut self, turn_meter: f32) {
        self.turn_meter+= turn_meter
    }

    //pub fn progress_turn_meter(&mut self, time: f32) {
    //    self.turn_meter+= (self.get_speed() * time);
    //    debug!("{} turn_meter progressed to {}", self, self.turn_meter);
    //}

    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }

    //pub fn get_speed(&self) -> f32 {
    //    let mut fact = 1.0;
    //    if self.has_effect(Effect::SpeedUpI) {
    //        fact *= 1.2;
    //    }
    //    if self.has_effect(Effect::SpeedDownII) {
    //        fact *= 0.6;
    //    }
    //    self.hero.speed  * fact 
    //}

    pub fn get_hero(&self) -> &Hero {
        &self.hero
    }

    pub fn get_health(&self) -> f32 {
        self.health
    }

    pub fn get_turn_meter(&self) -> f32 {
        self.turn_meter
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_instance() {
        // create a Hero
        let h : Hero = Hero {
            id:1,
            name:"Elhain".to_string(),
            health:15000.,
            attack:1000.,
            defense:1000.,
            speed:100.,
            crit_rate:0.0,
            crit_damage:0.0,
            effect_hit:0.0,
            effect_resistance:0.0,
            mastery:0.0,
            healing_effect:0.15,
            leech:0.15,
            piercing:0.15,
            tenacity:0.15,
            damage_reflection : 0.0,
            skills : Vec::new(),
            passives : Vec::new(),
        };
        let hi : Instance = Instance::new(&h,0,1,0);
        assert_eq!(h.health, hi.health);

    }

}
