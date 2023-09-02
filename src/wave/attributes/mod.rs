use crate::{debug, indent, data::{effect::Effect, skill::{Skill, get_targets, },  instance::Instance, faction::Faction, mark::Mark, hero::Hero}};

use super::{InstanceIndex, Wave};

pub mod crit;
pub mod heal;
pub mod attack;
pub mod defense;
pub mod mastery;

impl<const LEN:usize> Wave<'_,LEN> {

    pub fn get_turn_meter_reduction_reduction(&self,actor: InstanceIndex) -> f32 {
        let base = 0.0;
        if self.get_faction(actor) == Faction::TheForgotten {
            let nfact = self.get_bond(actor,Faction::TheForgotten);
            if nfact> 0.0 {
                let fact = 0.05 + (nfact-1.0)*0.025;
                debug!("{} has {} bond with TheForgotten -> turn_meter_reduction_reduction + {}", self.fmt(actor), nfact, fact);
                return base + fact;
            }
            base
        }
        else {
            base
        }
    }


    pub fn get_faction(&self, actor : InstanceIndex) -> Faction {
        self.heroes[actor].faction
    }

    pub fn get_mark(&self, actor : InstanceIndex) -> Mark{
        self.heroes[actor].mark
    }

    pub fn get_leech(&self, actor : InstanceIndex) -> f32 {
        let mut fact = 1.0;
        debug!("{} base leech of {}", self.name(actor),self.get_hero(actor).leech);
        indent!({
            if self.get_faction(actor) == Faction::SunsetSages{
                let xfact = self.get_bond(actor,Faction::SunsetSages);
                debug!("{} has {} bond with SunsetSages -> leech * {}", self.name(actor), xfact, xfact);
                fact *= xfact;

            }
        });
        if fact != 1.0 {
            debug!("{} leech of {}", self.name(actor), self.get_hero(actor).leech * fact);
        }
        self.heroes[actor].leech *fact
    }


    pub fn get_damage_reflect(&self,actor : InstanceIndex) -> f32 {
        self.heroes[actor].damage_reflection
    }



    pub fn get_effect_hit(&self, actor: InstanceIndex) -> f32 {
        // TODO handle effect hit buff/debuff
        let mut fact = 1.0;
        debug!("{} base effect hit of {}", self.name(actor),self.get_hero(actor).effect_hit);
        indent!({
            if self.heroes[actor].faction == Faction::WizardsEye {
                let xfact = self.get_bond(actor,Faction::WizardsEye);
                debug!("{} has {} bond with WizardsEye -> effect_hit * {}", self.name(actor), xfact, xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::EffectHitUpI) {
                let xfact = 1.25;
                debug!("{} has EffectHitUpI -> effect_hit * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::EffectHitUpII) {
                let xfact = 1.5;
                debug!("{} has EffectHitUpII -> effect_hit * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::EffectHitDownI) {
                let xfact = 0.75;
                debug!("{} has EffectHitDownI -> effect_hit * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::EffectHitDownII) {
                let xfact = 0.5;
                debug!("{} has EffectHitDownII -> effect_hit * {}", self.name(actor), xfact);
                fact *= xfact;
            }
        });
        let res = self.get_hero(actor).effect_hit * fact;
        if fact != 1.0 {
            debug!("{} effect hit of {}", self.name(actor), res);
        }
        res
    }


    pub fn is_alive(&self, actor : InstanceIndex) -> bool {
        self.health[actor] > 0.0
    }
    pub fn is_dead(&self, actor : InstanceIndex) -> bool {
        !self.is_alive(actor)
    }

    #[inline]
    pub fn get_hero(&self, actor : InstanceIndex) -> &Hero {
        &self.heroes[actor]
    }

    pub fn get_max_health(&self, actor : InstanceIndex) -> f32 {
        self.get_hero(actor).health
    }

    pub fn get_speed(&self, actor : InstanceIndex) -> f32 {
        let mut fact = 1.0;
        debug!("{} base speed of {}", self.name(actor),self.get_hero(actor).speed);
        indent!({
            if self.effects[actor].has(Effect::SpeedUpI) {
                let xfact = 1.2;
                debug!("{} has SpeedUpI -> speed * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::SpeedDownI) {
                let xfact = 0.8;
                debug!("{} has SpeedDownI -> speed * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::SpeedDownII) {
                let xfact = 0.6;
                debug!("{} has SpeedDownII -> speed * {}", self.name(actor), xfact);
                fact *= xfact;
            }
        });
        let res = self.get_hero(actor).speed  * fact ;
        if fact != 1.0 {
            debug!("{} speed of {}", self.name(actor), res);
        }
        res
    }

    pub fn get_effect_resistance(&self,actor : InstanceIndex) -> f32 {
        // TODO handle effect resistance buff/debuff
        let mut fact = 1.0;
        debug!("{} base effect resistance of {}", self.name(actor),self.get_hero(actor).effect_resistance);
        indent!({
            if self.effects[actor].has(Effect::EffectResistanceDownII) {
                let xfact = 0.5;
                debug!("{} has EffectResistanceDownII -> effect_resistance * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::EffectResistanceDownI) {
                let xfact = 0.75;
                debug!("{} has EffectResistanceDownI -> effect_resistance * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::EffectResistanceUpI) {
                let xfact = 1.25;
                debug!("{} has EffectResistanceUpI -> effect_resistance * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::EffectResistanceUpII) {
                let xfact = 1.5;
                debug!("{} has EffectResistanceUpII -> effect_resistance * {}", self.name(actor), xfact);
                fact *= xfact;
            }
        });
        let res = self.get_hero(actor).effect_resistance * fact;
        if fact != 1.0 {
            debug!("{} effect resistance of {}", self.name(actor), res);
        }
        res
    }


    
    pub fn get_attack_damage(&self,actor : InstanceIndex) -> f32 {
        self.get_attack(actor) 
    }


    pub fn get_tenacity(&self,actor:InstanceIndex) -> f32 {
        let mut fact = 1.0;
        debug!("{} base tenacity of {}", self.name(actor),self.get_hero(actor).tenacity);
        indent!({
            if self.effects[actor].has(Effect::TenacityUpI) {
                let xfact = 1.3;
                debug!("{} has TenacityUpI -> tenacity * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::TenacityUpII) {
                let xfact = 1.6;
                debug!("{} has TenacityUpII -> tenacity * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::TenacityDownI) {
                let xfact = 0.7;
                debug!("{} has TenacityDownI -> tenacity * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::TenacityDownII) {
                let xfact = 0.4;
                debug!("{} has TenacityDownII -> tenacity * {}", self.name(actor), xfact);
                fact *= xfact;
            }
        });
        let res = self.get_hero(actor).tenacity *fact;
        if fact != 1.0 {
            debug!("{} tenacity of {}", self.name(actor), res);
        }
        res
    }

}