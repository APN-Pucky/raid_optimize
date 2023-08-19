// Import (via `use`) the `fmt` module to make it available.
use std::collections::HashMap;
use std::fmt;
use log::debug;

use rand::seq::SliceRandom;
use rand::Rng;

use crate::hero::Hero;
use crate::wave::{Wave, InstanceRef};
use crate::hero::skill::Skill;
use crate::hero::effect::Effect;

use super::skill::get_cooldown;

#[derive(Debug)]
pub struct Instance {
    pub hero: Hero,
    pub id : u32,
    pub iref : InstanceRef,
    pub health: u32,
    pub shield: u32,
    pub turn_meter: u32,
    pub cooldowns : Vec<u32>,
    pub statistics: HashMap<String, u32>,    
    pub effects : Effects,
}

#[derive(Debug)]
pub struct Effects {
    pub hm : HashMap<Effect, Vec<(u32,InstanceRef)>>,
}

impl Effects {
    pub fn new() -> Effects {
        Effects {
            hm : HashMap::new(),
        }
    }

    pub fn get(&self, key: Effect) -> u32 {
        match self.hm.get(&key) {
            Some(v) => v.len() as u32,
            None => 0,
        }
    }

    pub fn push(&mut self, key: Effect, turns : u32, ir:InstanceRef) {
        match self.hm.get_mut(&key) {
            Some(v) => v.push((turns,ir)),
            None => {
                let mut v = Vec::new();
                v.push((turns,ir));
                self.hm.insert(key, v);
            }
        }
    }

    pub fn reduce(&mut self) {
        for (k,v) in self.hm.iter_mut() {
            reduce_effect(v);
        }
    }
}

pub fn reduce_effect(e : &mut Vec<(u32,InstanceRef)>) {
    for (j,i) in e.iter_mut() {
        *j -= 1;
    }
    e.retain(|&(x,i)| x > 0);
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{} [health: {}, turn_meter: {}]", self.hero.name, self.id,self.health, self.turn_meter)
    }
}

impl PartialEq for Instance {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Instance {
    pub fn new(hero: &Hero, id:u32, iref: InstanceRef) -> Instance {
        Instance {
            hero: hero.clone(),
            id,
            iref,
            health: hero.health,
            shield: 0,
            turn_meter: 0,
            cooldowns : hero.skills.iter().map(|_| 0).collect(),
            statistics: HashMap::new(),
            effects : Effects::new(), 
        }
    }

    pub fn cooldown(&mut self, skill :Skill) {
        // find index of skill in hero.skills
        self.hero.skills.iter().position(|s| *s == skill).map(|i| self.cooldowns[i] = get_cooldown(skill));
    }
    
    pub fn reduce_cooldowns(&mut self) {
        for i in 0..self.cooldowns.len() {
            if self.cooldowns[i] > 0 {
                self.cooldowns[i] -= 1;
            }
        }
    }

    pub fn get_cooldown_mask(&self) -> Vec<bool> {
        self.cooldowns.iter().map(|c| *c == 0).collect()
    }

    pub fn get_active_skills(&self) -> Vec<Skill> {
        self.hero.skills.iter().zip(self.get_cooldown_mask()).filter(|(_,c)| *c).map(|(s,_)| s.clone()).collect()
    }

    pub fn heal(&mut self, health: u32) {
        let mut heal = (health as f32 * self.hero.healing_effect) as u32; // TODO handle rounding
        log::debug!("{} heals {} health", self, heal);
        self.add_stat("health healed", heal);
        self.health += heal;
    }

    pub fn add_stat(&mut self, key: &str, statistics: u32 ) {
        *self.statistics.entry(key.to_string()).or_insert(0) += statistics;
    }

    pub fn copy_statistics(&self) -> HashMap<String, u32> {
        self.statistics.clone()
    }

    pub fn get_statistics(&self) -> &HashMap<String, u32>{
        &self.statistics
    }

    pub fn get_defense(&self) -> u32 {
        // TODO handle defense buff/debuff
        self.hero.defense
    }

    #[deprecated(note="use `get_attack_damage` instead")]
    pub fn get_attack(&self) -> u32 {
        self.get_attack_damage()
    }

    pub fn get_attack_damage(&self) -> u32 {
        // TODO handle attack buff/debuff
        self.hero.attack
    }

    pub fn get_max_health(&self) -> u32 {
        // TODO handle max hp buff/debuff
        self.hero.health
    }

    pub fn has_effect(&self, key: Effect) -> bool {
        match self.effects.hm.get(&key) {
            Some(v) => v.len() > 0,
            None => false,
        }
    }


    pub fn reduce_effects(&mut self) {
        self.effects.reduce();
    }

    pub fn reduce_turn_meter(&mut self, target:&mut Instance, turn_meter_reduction_ratio: f32) {
        let mut turn_meter = (target.get_turn_meter() as f32 * turn_meter_reduction_ratio ) as u32; // TODO handle rounding ;
        target.set_turn_meter(turn_meter);
        log::debug!("{} reduces turn meter of {} by {} to {}", self, target, turn_meter_reduction_ratio, turn_meter);
        self.add_stat("turn meter reduced", turn_meter);
    }

    pub fn inflict(&mut self, target : &mut Instance, effect: Effect, chance : f32, turns: u32) {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < chance {
            log::debug!("{} inflicts {} for {} on {}", self, effect, turns, target);
            self.add_stat(&format!("{} inflicted",effect), turns);
            target.effects.push(effect, turns, self.iref);
        }
    }

    pub fn inflict_bleed(&mut self, target : &mut Instance, bleed_chance: f32,bleed_turns: u32) {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < bleed_chance {
            let n = target.effects.get(Effect::Bleed);
            if n < 10 {
                log::debug!("{} inflicts {} bleed on {}", self, bleed_turns, target);
                self.add_stat("bleed inflicted", 1);
                target.effects.push(Effect::Bleed, bleed_turns,self.iref );
                let dmg_vec = vec![0.14,0.18,0.22,0.26,0.30,0.30,0.30,0.30,0.30,0.30];
                let bleed_dmg = (self.get_attack_damage()as f32 * dmg_vec[(n) as usize]) as u32;
                target.take_bleed_damage(bleed_dmg);
            }
            else {
                log::debug!("{} already has 10 bleed", target);
            }
        }
    }

    pub fn take_bleed_damage(&mut self, bleed_dmg: u32) {
        log::debug!("{} takes {} damage from bleed", self, bleed_dmg);
        // todo handle mastery
        self.take_damage(bleed_dmg);
    }

    pub fn after_action(&mut self) {
        // TODO handle effects
        self.reduce_effects();
    }


    pub fn loose_shield(&mut self, damage: u32) -> u32 {
        if self.shield > damage {
            log::debug!("{} looses {} shield", self, damage);
            self.add_stat("shield lost", damage);
            self.shield -= damage;
            return 0;
        }
        else if self.shield == 0 {
            return damage;
        }
        else {
            log::debug!("{} looses all {} shield", self, self.shield);
            self.add_stat("shield lost", self.shield);
            self.shield = 0;
            return damage - self.shield;
        }
    }

    pub fn loose_health(&mut self, damage: u32) {
        log::debug!("{} looses {} health", self, damage);
        if self.health < damage {
            self.add_stat("health lost", self.health);
            self.health = 0;
            return;
        }
        else {
            self.add_stat("health lost", damage);
            self.health -= damage;
        }
    }

    pub fn take_damage(&mut self, damage: u32) {
        debug!("{} takes {} damage", self, damage);
        self.add_stat("damage taken", damage);
        let dmg = self.loose_shield(damage);
        self.loose_health(dmg);
    }

    pub fn deal_damage(&mut self, target: &mut Instance, damage: u32) {
        debug!("{} takes {} damage from {}", target , damage , self);
        self.add_stat("damage done", damage);
        target.take_damage(damage);
        let leech = (damage as f32 * self.hero.leech) as u32; // TODO handle rounding
        self.add_stat("leeched", leech);
        self.heal(leech);
    }

    pub fn attack(&mut self, target: &mut Instance, atk_dmg:u32 ) {
        // test if critical strike
        self.add_stat("attacks", 1);
        let mut rng = rand::thread_rng();
        let crit = rng.gen::<f32>() < self.hero.crit_rate;
        let mut attack  = atk_dmg; //(self.get_attack() as f32 * dmg_ratio) as u32;
        if crit {
            self.add_stat("critical strikes", 1);
            let mut crit = self.hero.crit_damage;
            let mut tenacity = target.hero.tenacity;
            if tenacity > crit {
                tenacity = crit;
            }
            let crit_rate = crit - tenacity;
            self.add_stat("critical damage", (attack as f32 * crit_rate ) as u32);
            target.add_stat("tenacity ignored", (attack as f32 * tenacity ) as u32);
            attack = (attack as f32 * crit_rate) as u32; // TODO handle rounding
            log::debug!("{} critical attack ({}%={}%-{}%)", self,crit_rate*100.,crit*100.,tenacity*100.);
        }
        log::debug!("{} attacks {} with {} attack", self, target,attack );
        self.add_stat("attack", attack);
        let mut def = target.get_defense();

        let pierce = (def as f32 * self.hero.piercing) as u32; // TODO handle rounding
        self.add_stat("pierced defense", pierce);
        log::debug!("{} pierces {} defense of {} ({}%)", self, pierce, def, self.hero.piercing*100.);
        def -= pierce;
        
        self.deal_damage(target, attack - def);
    }

    pub fn reset_turn_meter(&mut self) {
        self.set_turn_meter(0)
    }

    pub fn set_turn_meter(&mut self, turn_meter: u32) {
        self.turn_meter= turn_meter
    }

    pub fn increase_turn_meter(&mut self, time: f32) {
        self.turn_meter+= (self.get_speed() as f32 * time) as u32;
        log::debug!("{} turn_meter is now {}", self, self.turn_meter);
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn get_speed(&self) -> u32 {
        // TODO handle speed buff/debuff
        self.hero.speed
    }

    pub fn get_hero(&self) -> &Hero {
        &self.hero
    }

    pub fn get_health(&self) -> u32 {
        self.health
    }

    pub fn get_shield(&self) -> u32 {
        self.shield
    }

    pub fn get_turn_meter(&self) -> u32 {
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
            health:15000,
            attack:1000,
            defense:1000,
            speed:100,
            crit_rate:0.0,
            crit_damage:0.0,
            effect_hit:0.0,
            effect_resistance:0.0,
            mastery:0.0,
            healing_effect:0.15,
            leech:0.15,
            piercing:0.15,
            tenacity:0.15,
            skills : Vec::new(),
        };
        let hi : Instance = Instance::new(&h,0,InstanceRef{team:true,index:0});
        assert_eq!(h.health, hi.health);

    }

}
