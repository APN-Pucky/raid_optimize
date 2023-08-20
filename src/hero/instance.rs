// Import (via `use`) the `fmt` module to make it available.
use std::fmt;
use enum_map::EnumMap;
use log::debug;

use rand::seq::SliceRandom;
use rand::Rng;


use crate::hero::Hero;
use crate::hero::stat::effect_to_stat;
use crate::wave::{ InstanceRef, TURN_METER_THRESHOLD};
use crate::hero::skill::Skill;
use crate::hero::effect::Effect;

use super::skill::get_cooldown;
use super::stat::Stat;

#[derive(Debug)]
pub struct Instance {
    pub hero: Hero,
    pub id : u32,
    pub iref : InstanceRef,
    health: u32,
    shield: Vec<(u32,u32)>, // (shield, turns)
    turn_meter: u32,
    pub cooldowns : Vec<u32>,
    track_statistics: bool,
    pub statistics: EnumMap<Stat,u32>,
    pub effects : Effects,
}

#[derive(Debug)]
pub struct Effects {
    pub em : EnumMap<Effect,Vec<(u32,InstanceRef)>>,
    //pub vm : [Vec<(u32,InstanceRef)>;Effect::NumberOfEffects as usize],
}

impl Default for Effects {
    fn default() -> Self {
        Self::new()
    }
}

impl Effects {
    pub fn new() -> Effects {
        Effects {
            em : EnumMap::default(),
        }
    }

    pub fn get(&self, key: Effect) -> u32 {
        self.em[key].len() as u32
    }

    pub fn push(&mut self, key: Effect, turns : u32, ir:InstanceRef) {
        self.em[key ].push((turns,ir));
    }

    pub fn remove_empty(&mut self) {
        // remove zero elements from effect vectors
        for (_key,value) in self.em.iter_mut() {
            value.retain(|&(x,_)| x > 0);
        }
    }

    pub fn reduce(&mut self) {
        for (_key,value) in self.em.iter_mut() {
            let mut i = 0;
            while i < value.len() {
                value[i].0 -= 1;
                if value[i].0 == 0 {
                    value.remove(i);
                }
                else {
                    i += 1;
                }
            } 
        }
        //self.remove_empty();
    }
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
    pub fn new(hero: &Hero, id:u32, iref: InstanceRef , track_statistics : bool) -> Instance {
        Instance {
            hero: hero.clone(),
            id,
            iref,
            health: hero.health,
            shield: Vec::new(),
            turn_meter: 0,
            cooldowns : hero.skills.iter().map(|_| 0).collect(),
            track_statistics,
            statistics: EnumMap::default(),
            effects : Effects::new(), 
        }
    }

    pub fn cooldown(&mut self, skill :Skill) {
        // find index of skill in hero.skills
        if let Some(i) = self.hero.skills.iter().position(|s| *s == skill) {
            self.cooldowns[i] = get_cooldown(skill);
        }
        else {
            panic!("Skill {:?} not found in hero {:?}", skill, self.hero);
        }
    }
    
    pub fn reduce_cooldowns(&mut self) {
        for i in 0..self.cooldowns.len() {
            if self.cooldowns[i] > 0 {
                self.cooldowns[i] -= 1;
            }
        }
    }

    #[deprecated]
    pub fn get_cooldown_mask(&self) -> Vec<bool> {
        self.cooldowns.iter().map(|c| *c == 0).collect()
    }

    pub fn get_active_skills(&self) -> Vec<Skill> {
        let mut ret  = Vec::new();
        for i in 0..self.cooldowns.len() {
            if self.cooldowns[i] == 0 {
                ret.push(self.hero.skills[i]);
            }
        }
        ret
        //self.hero.skills.iter().zip(self.get_cooldown_mask()).filter(|(_,c)| *c).map(|(s,_)| *s).collect()
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

    pub fn restore(&mut self, target: &mut Instance, heal: u32 ) {
        let heal = (heal as f32 * self.hero.healing_effect) as u32; // TODO handle rounding
        log::debug!("{} restores {} health to {}", self, heal, target);
        self.add_stat(Stat::HealthRestored, heal);
        target.heal(heal);
    }

    pub fn heal(&mut self, health: u32) {
        if ! self.health > 0 {
            log::warn!("{} is dead, cannot heal", self);
            return;
        }
        let heal = (health as f32 * self.hero.healing_effect) as u32; // TODO handle rounding
        log::debug!("{} heals {} health", self, heal);
        self.add_stat(Stat::HealthHealed, heal);
        self.health += heal;
    }

    pub fn add_stat(&mut self, key: Stat, statistics: u32 ) {
        if self.track_statistics {
            self.statistics[key] += statistics;
        }
    }

    //pub fn copy_statistics(&self) -> EnumMap<Stat, u32> {
    //    self.statistics.clone()
    //}

    pub fn get_defense(&self) -> u32 {
        // TODO handle defense buff/debuff
        self.hero.defense
    }

    pub fn get_attack(&self) -> u32 {
        // TODO handle attack buff/debuff
        if self.has_effect(Effect::AttackUpII) {
            (self.hero.attack * 140) / 100
        }
        else {
            self.hero.attack
        }
    }

    pub fn get_attack_damage(&self) -> u32 {
        self.get_attack() 
    }

    pub fn get_max_health(&self) -> u32 {
        // TODO handle max hp buff/debuff
        self.hero.health
    }

    pub fn has_effect(&self, key: Effect) -> bool {
        !self.effects.em[key].is_empty()
    }

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
        let mut red = (TURN_METER_THRESHOLD as f32 * turn_meter_reduction_ratio)as u32;
        let turn = target.get_turn_meter();
        if red > turn {
            red  = turn
        }
        let turn_meter = turn - red;
        log::debug!("{} reduces turn meter of {} by {} to {}", self, target, turn_meter_reduction_ratio, turn_meter);
        target.set_turn_meter(turn_meter);
        self.add_stat(Stat::TurnMeterReduced, turn_meter);
    }

    pub fn get_inflicted(&mut self, iref: &InstanceRef, effect: Effect,chance : f32, turns:u32) {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < chance {
            self.effects.push(effect, turns, *iref);
        }
    }

    pub fn inflict(&mut self, target : &mut Instance, effect: Effect, chance : f32, turns: u32) {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < chance {
            log::debug!("{} inflicts {} for {} on {}", self, effect, turns, target);
            //self.add_stat(&format!("{} inflicted",effect), turns);
            self.add_stat(effect_to_stat(effect), turns);
            target.effects.push(effect, turns, self.iref);
        }
    }

    pub fn inflict_hp_burning(&mut self, target : &mut Instance, chance: f32,turns: u32) {
        let n = target.effects.get(Effect::HPBurning);
        if n < 5 {       
            self.inflict(target, Effect::HPBurning, chance,turns);
        }
    }

    pub fn take_hp_burning_damage(&mut self, dmg: u32) {
        log::debug!("{} takes {} damage from hp_burning", self, dmg);
        // todo handle mastery
        self.take_damage(dmg);
    }

    pub fn inflict_bleed(&mut self, target : &mut Instance, bleed_chance: f32,bleed_turns: u32) {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < bleed_chance {
            let n = target.effects.get(Effect::Bleed);
            if n < 10 {
                log::debug!("{} inflicts {} bleed on {}", self, bleed_turns, target);
                self.add_stat(effect_to_stat(Effect::Bleed), bleed_turns);
                target.effects.push(Effect::Bleed, bleed_turns,self.iref );
                let dmg_vec = vec![14,18,22,26,30,30,30,30,30,30];
                let bleed_dmg = (self.get_attack_damage() * dmg_vec[(n) as usize])/100;
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

    pub fn get_shield(&self) -> u32 {
        let mut shield = 0;
        for (s,_) in self.shield.iter() {
            shield += s;
        }
        shield
    }

    pub fn subtract_shield(&mut self, var:u32) {
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

    pub fn add_shield(&mut self, value:u32,turns:u32) {
        self.shield.push((value,turns));
    }

    pub fn loose_shield(&mut self, damage: u32) -> u32 {
        let current_shield = self.get_shield();
        if current_shield > damage {
            log::debug!("{} looses {} shield", self, damage);
            self.subtract_shield(damage);
            0
        }
        else if current_shield == 0 {
            damage
        }
        else { // damage > shield
            log::debug!("{} looses all {} shield", self, current_shield);
            self.add_stat(Stat::ShieldBlocked, current_shield);
            self.shield = Vec::new();
            damage - current_shield
        }
    }

    pub fn loose_health(&mut self, damage: u32) {
        log::debug!("{} looses {} health", self, damage);
        if self.health < damage {
            self.add_stat(Stat::HealthLost, self.health);
            self.health = 0;
        }
        else {
            self.add_stat(Stat::HealthLost, damage);
            self.health -= damage;
        }
    }

    pub fn take_damage(&mut self, damage: u32) {
        debug!("{} takes {} damage", self, damage);
        self.add_stat(Stat::DamageTaken, damage);
        let dmg = self.loose_shield(damage);
        self.loose_health(dmg);
    }

    pub fn deal_damage(&mut self, target: &mut Instance, damage: u32) {
        debug!("{} takes {} damage from {}", target , damage , self);
        self.add_stat(Stat::DamageDone, damage);
        target.take_damage(damage);
        let leech = (damage as f32 * self.hero.leech) as u32; // TODO handle rounding
        if leech > 0 {
            self.add_stat(Stat::Leeched, leech);
            self.heal(leech);
        }
    }

    pub fn attack(&mut self, target: &mut Instance, atk_dmg:u32 ) {
        // test if critical strike
        self.add_stat(Stat::Attacks, 1);
        let mut rng = rand::thread_rng();
        let crit = rng.gen::<f32>() < self.hero.crit_rate;
        let mut attack  = atk_dmg; //(self.get_attack() as f32 * dmg_ratio) as u32;
        if crit {
            self.add_stat(Stat::CriticalStrikes, 1);
            let crit = self.hero.crit_damage;
            let mut tenacity = target.hero.tenacity;
            if tenacity > crit {
                tenacity = crit;
            }
            let crit_rate = crit - tenacity;
            self.add_stat(Stat::CriticalDamage, (attack as f32 * crit_rate ) as u32);
            target.add_stat(Stat::TenacityIgnored, (attack as f32 * tenacity ) as u32);
            attack = (attack as f32 * crit_rate) as u32; // TODO handle rounding
            log::debug!("{} critical attack ({}%={}%-{}%)", self,crit_rate*100.,crit*100.,tenacity*100.);
        }
        log::debug!("{} attacks {} with {} attack", self, target,attack );
        self.add_stat(Stat::Attack, attack);
        let mut def = target.get_defense();

        let pierce = (def as f32 * self.hero.piercing) as u32; // TODO handle rounding
        self.add_stat(Stat::PiercedDefense, pierce);
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

    pub fn increase_turn_meter(&mut self, turn_meter: u32) {
        self.turn_meter+= turn_meter
    }

    pub fn decrease_turn_meter(&mut self, turn_meter: u32) {
        self.turn_meter+= turn_meter
    }

    pub fn progress_turn_meter(&mut self, time: f32) {
        self.turn_meter+= (self.get_speed() as f32 * time) as u32;
        log::debug!("{} turn_meter is now {}", self, self.turn_meter);
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn get_speed(&self) -> u32 {
        let mut fact = 1.0;
        // TODO handle speed buff/debuff
        if self.has_effect(Effect::SpeedUpI) {
            fact *= 1.2;
        }
        if self.has_effect(Effect::SpeedDownII) {
            fact *= 0.6;
        }
        (self.hero.speed as f32 * fact ) as  u32
    }

    pub fn get_hero(&self) -> &Hero {
        &self.hero
    }

    pub fn get_health(&self) -> u32 {
        self.health
    }

    pub fn get_turn_meter(&self) -> u32 {
        self.turn_meter
    }

    pub fn get_basic_attack_damage(&self) -> u32 {
        if self.has_effect(Effect::RippleII) {
            (self.get_attack() * 140) / 100
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
            damage_reflection : 0.0,
            skills : Vec::new(),
        };
        let hi : Instance = Instance::new(&h,0,InstanceRef{team:true,index:0},false);
        assert_eq!(h.health, hi.health);

    }

}
