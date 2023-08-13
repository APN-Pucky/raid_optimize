// Import (via `use`) the `fmt` module to make it available.
use std::collections::HashMap;
use std::fmt;
use log::debug;

use rand::seq::SliceRandom;
use rand::Rng;

use crate::hero::Hero;
use crate::wave::Wave;

#[derive(Debug)]
pub struct Instance {
    hero: Hero,
    id : u32,
    health: u32,
    shield: u32,
    initiative: u32,
    statistics: HashMap<String, u32>,    
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{} [health: {}, initiative: {}]", self.hero.name, self.id,self.health, self.initiative)
    }
}

impl PartialEq for Instance {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Instance {
    pub fn new(hero: &Hero, id:u32) -> Instance {
        Instance {
            hero: hero.clone(),
            id,
            health: hero.health,
            shield: 0,
            initiative: 0,
            statistics: HashMap::new(),
        }
    }

    pub fn add_stat(&mut self, key: &str, statistics: u32 ) {
        *self.statistics.entry(key.to_string()).or_insert(0) += statistics;
    }

    pub fn get_statistics(&self) -> &HashMap<String, u32>{
        &self.statistics
    }

    pub fn get_defense(&self) -> u32 {
        // TODO handle defense buff/debuff
        self.hero.defense
    }

    pub fn get_attack(&self) -> u32 {
        // TODO handle attack buff/debuff
        self.hero.attack
    }

    //pub fn choose_target(& self, opponents: &mut Vec<Instance>) -> Option<&mut &mut Instance> {
    //    // TODO handle target selection
    //    opponents.iter_mut().filter(|i| i.is_alive()).collect::<Vec<&mut Instance>>().choose_mut(&mut rand::thread_rng())
    //}

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
    }

    pub fn attack(&mut self, target: &mut Instance) {
        // test if critical strike
        self.add_stat("attacks", 1);
        let mut rng = rand::thread_rng();
        let crit = rng.gen::<f32>() < self.hero.crit_rate;
        let mut attack  = self.get_attack();
        if crit {
            log::debug!("{} critical strike", self);
            self.add_stat("critical strikes", 1);
            attack = (attack as f32 * self.hero.crit_damage) as u32; // TODO handle rounding
        }
        log::debug!("{} attacks {} with {} attack", self, target,attack );
        self.deal_damage(target, attack - target.get_defense());
    }

    pub fn reset_initiative(&mut self) {
        self.set_initiative(0)
    }

    pub fn set_initiative(&mut self, initiative: u32) {
        self.initiative = initiative
    }

    pub fn increase_initiative(&mut self) {
        self.initiative += self.get_speed();
        log::debug!("{} initiative is now {}", self, self.initiative);
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

    pub fn get_initiative(&self) -> u32 {
        self.initiative
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_instance() {
        // create a Hero
        let h : Hero = Hero{
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
        };
        let hi : Instance = Instance::new(&h,0,);
        assert_eq!(h.health, hi.health);

    }

}
