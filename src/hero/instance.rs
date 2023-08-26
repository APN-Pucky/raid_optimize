// Import (via `use`) the `fmt` module to make it available.
use std::fmt;
use enum_map::EnumMap;

use rand::seq::SliceRandom;
use rand::Rng;


use crate::hero::Hero;
use crate::hero::stat::effect_to_stat;
use crate::{roll, indent, debug, warn, info, error};
use crate::wave::{ InstanceRef, TURN_METER_THRESHOLD};
use crate::hero::skill::Skill;
use crate::hero::effects::Effects;

use super::effects::buff::Buff;
use super::effects::buff::attribute::AttributeBuff;
use super::effects::debuff::Debuff;
use super::effects::debuff::dot::DotDebuff;
use super::passive::Passive;
use super::skill::get_cooldown;
use super::stat::Stat;


#[derive(Debug)]
pub struct Instance<'a> {
    pub hero:  &'a Hero,
    pub id : u32,
    pub iref : InstanceRef,
    health: f32,
    shield: Vec<(f32,u32)>, // (shield, turns)
    turn_meter: f32,
    pub cooldowns : Vec<u32>,
    track_statistics: bool,
    pub statistics: EnumMap<Stat,f32>,
    pub effects: Effects, 
    pub passives : Vec<Passive>
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
    pub fn new(hero: &Hero, id:u32, iref: InstanceRef , track_statistics : bool) -> Instance {
        Instance {
            hero: hero,
            id,
            iref,
            health: hero.health,
            shield: Vec::new(),
            turn_meter: 0.0,
            cooldowns : hero.skills.iter().map(|_| 0).collect(),
            track_statistics,
            statistics: EnumMap::default(),
            effects : Effects::new(), 
            passives : hero.passives.clone(),
        }
    }

    pub fn has_passive(&self, passive: Passive) -> bool {
        self.passives.iter().any(|p| *p == passive)
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


    pub fn cleanse<F>(&mut self, effect_closure:&F, layers: u32) where F : Fn(Effect) -> bool {
        for (k,v) in self.effects.em.iter_mut() {
            if effect_closure(k) {
                // drop `layers` randomly of v
                if v.len() > layers as usize {
                    let mut rng = rand::thread_rng();
                    //let mut v = v.clone();
                    v.shuffle(&mut rng);
                    for _i in 0..layers {
                        v.pop();
                    }
                }
                else {
                    // empty v
                    v.clear();
                }
            }
        }
        self.effects.remove_empty();
    }

    pub fn restore(&mut self, target: &mut Instance, heal: f32 ) {
        //let heal = heal * self.hero.healing_effect; 
        debug!("{} restores {} health to {}", self, heal, target);
        self.add_stat(Stat::HealthRestored, heal);
        target.heal(heal);
    }

    pub fn is_dead(&self) -> bool {
        !self.is_alive()
    }

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

    pub fn add_stat(&mut self, key: Stat, statistics: f32 ) {
        if self.track_statistics {
            self.statistics[key] += statistics;
        }
    }

    //pub fn copy_statistics(&self) -> EnumMap<Stat, u32> {
    //    self.statistics.clone()
    //}

    pub fn get_defense(&self) -> f32 {
        // TODO handle defense buff/debuff
        self.hero.defense
    }

    pub fn get_attack(&self) -> f32 {
        // TODO handle attack buff/debuff
        if self.effects.attribute_buffs.has(AttributeBuff::AttackUpII) {
            self.hero.attack  * 1.4
        }
        else {
            self.hero.attack 
        }
    }

    pub fn get_attack_damage(&self) -> f32 {
        self.get_attack() 
    }

    pub fn get_max_health(&self) -> f32 {
        // TODO handle max hp buff/debuff
        self.hero.health
    }


    //pub fn has_effect(&self, key: Effect) -> bool {
    //    !self.effects.em[key].is_empty()
    //}

    pub fn reduce_shields(&mut self) {
        let mut i = 0;
        while i < self.shield.len() {
            self.shield[i].1 -= 1;
            if self.shield[i].1 == 0 {
                self.shield.remove(i);
            }
            else {
                i += 1;
            }
        }
    }

    pub fn reduce_effects(&mut self) {
        self.effects.reduce();
    }

    pub fn reduce_turn_meter(&mut self, target:&mut Instance, turn_meter_reduction_ratio: f32) {
        // relative to total!
        let mut red = (TURN_METER_THRESHOLD * turn_meter_reduction_ratio);
        let turn = target.get_turn_meter();
        if red > turn {
            red  = turn
        }
        let turn_meter = turn - red;
        debug!("{} reduces turn meter of {} by {} to {}", self, target, turn_meter_reduction_ratio, turn_meter);
        target.set_turn_meter(turn_meter);
        //self.add_stat(Stat::TurnMeterReduced, turn_meter);
    }

    pub fn get_inflicted_buff(&mut self, iref: &InstanceRef, effect: Buff,chance : f32, turns:u32) {
        if roll(chance) {
            self.effects.buffs.push(effect, turns, *iref);
        }
    }

    pub fn get_inflicted_debuff(&mut self, iref: &InstanceRef, effect: Debuff,chance : f32, turns:u32) {
        if roll(chance) {
            self.effects.debuffs.push(effect, turns, *iref);
        }
    }

    pub fn inflict(&mut self, target : &mut Instance, effect: Effect, chance : f32, turns: u32) {
        if roll(chance) {
            debug!("{} inflicts {} for {} on {}", self, effect, turns, target);
            //self.add_stat(&format!("{} inflicted",effect), turns);
            self.add_stat(effect_to_stat(effect), turns as f32);
            target.effects.push(effect, turns, self.iref);
        }
    }

    pub fn inflict_hp_burning(&mut self, target : &mut Instance, chance: f32,turns: u32) {
        let n = target.effects.get(Effect::HPBurning);
        if n < 5 {       
            self.inflict(target, Effect::HPBurning, chance,turns);
        }
    }

    pub fn take_hp_burning_damage(&mut self, dmg: f32) {
        debug!("{} takes {} damage from hp_burning", self, dmg);
        // todo handle mastery
        self.take_damage(dmg);
    }

    pub fn inflict_bleed(&mut self, target : &mut Instance, bleed_chance: f32,bleed_turns: u32) {
        if roll(bleed_chance) {
            let n = target.effects.get(Effect::Bleed);
            if n < 10 {
                debug!("{} inflicts {} bleed on {}", self, bleed_turns, target);
                self.add_stat(effect_to_stat(Effect::Bleed), bleed_turns as f32);
                target.effects.push(Effect::Bleed, bleed_turns,self.iref );
                let dmg_vec = vec![0.14,0.18,0.22,0.26,0.30,0.30,0.30,0.30,0.30,0.30];
                let bleed_dmg = (self.get_attack_damage() * dmg_vec[(n) as usize]);
                target.take_bleed_damage(bleed_dmg);
            }
            else {
                debug!("{} already has 10 bleed", target);
            }
        }
    }

    pub fn take_bleed_damage(&mut self, bleed_dmg: f32) {
        debug!("{} takes {} damage from bleed", self, bleed_dmg);
        // todo handle mastery
        self.take_damage(bleed_dmg);
    }

    pub fn get_shield(&self) -> f32 {
        let mut shield: f32 = 0.0;
        for (s,_) in self.shield.iter() {
            shield += s;
        }
        shield
    }

    pub fn subtract_shield(&mut self, var:f32) {
        self.add_stat(Stat::ShieldBlocked, var);
        let mut value = var;
        let i = 0;
        while i < self.shield.len() {
            if self.shield[i].0 > value {
                self.shield[i].0 -= value;
                return;
            }
            else {
                value -= self.shield[i].0;
                self.shield.remove(i);
            }
        }
    }

    pub fn add_shield(&mut self, value:f32,turns:u32) {
        self.shield.push((value,turns));
    }

    pub fn loose_shield(&mut self, damage: f32) -> f32 {
        let current_shield = self.get_shield();
        if current_shield > damage {
            debug!("{} looses {} shield", self, damage);
            self.subtract_shield(damage);
            0.0
        }
        else if current_shield == 0.0 {
            damage
        }
        else { // damage > shield
            debug!("{} looses all {} shield", self, current_shield);
            self.add_stat(Stat::ShieldBlocked, current_shield);
            self.shield = Vec::new();
            damage - current_shield
        }
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

    pub fn progress_turn_meter(&mut self, time: f32) {
        self.turn_meter+= (self.get_speed() * time);
        debug!("{} turn_meter progressed to {}", self, self.turn_meter);
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }

    pub fn get_speed(&self) -> f32 {
        let mut fact = 1.0;
        if self.has_effect(Effect::SpeedUpI) {
            fact *= 1.2;
        }
        if self.has_effect(Effect::SpeedDownII) {
            fact *= 0.6;
        }
        self.hero.speed  * fact 
    }

    pub fn get_hero(&self) -> &Hero {
        &self.hero
    }

    pub fn get_health(&self) -> f32 {
        self.health
    }

    pub fn get_turn_meter(&self) -> f32 {
        self.turn_meter
    }

    pub fn get_basic_attack_damage(&self) -> f32 {
        if self.has_effect(Effect::RippleII) {
            self.get_attack() * 1.40
        }
        else {
            self.get_attack()
        }
    }

    pub fn get_effect_resistance(&self) -> f32 {
        // TODO handle effect resistance buff/debuff
        let mut fact = 1.0;
        if self.has_effect(Effect::EffectResistanceDownII) {
            fact = 0.5;
        }
        self.hero.effect_resistance * fact
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
        let hi : Instance = Instance::new(&h,0,InstanceRef{team:true,index:0},false);
        assert_eq!(h.health, hi.health);

    }

}
