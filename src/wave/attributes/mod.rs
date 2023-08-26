use crate::{debug, indent, hero::{effect::Effect, skill::{Skill, get_targets, execute_skill}, Hero, instance::Instance, faction::Faction}};

use super::{InstanceIndex, Wave};

pub mod crit;
pub mod heal;
pub mod attack;

impl<const LEN:usize> Wave<'_,LEN> {

    pub fn get_mastery(&self,actor : InstanceIndex) -> f32 {
        self.heroes[actor].mastery
    }

    pub fn get_damage_reflect(&self,actor : InstanceIndex) -> f32 {
        self.heroes[actor].damage_reflection
    }



    pub fn get_effect_hit(&self, actor: InstanceIndex) -> f32 {
        // TODO handle effect hit buff/debuff
        let mut fact = 1.0;
        debug!("{} base effect hit of {}", self.fmt(actor),self.get_hero(actor).effect_hit);
        indent!({
            if self.heroes[actor].faction == Faction::WizardsEye {
                let scale = vec![1.0,1.06,1.09,1.12,1.15];
                let xfact = scale[self.count_faction(actor,Faction::WizardsEye)-1];
                debug!("{} has {} WizardsEye allies -> effect_hit * {}", self.fmt(actor), self.count_faction(actor,Faction::WizardsEye), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::EffectHitUpI) {
                let xfact = 1.25;
                debug!("{} has EffectHitUpI -> effect_hit * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::EffectHitUpII) {
                let xfact = 1.5;
                debug!("{} has EffectHitUpII -> effect_hit * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::EffectHitDownI) {
                let xfact = 0.75;
                debug!("{} has EffectHitDownI -> effect_hit * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::EffectHitDownII) {
                let xfact = 0.5;
                debug!("{} has EffectHitDownII -> effect_hit * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
        });
        let res = self.get_hero(actor).effect_hit * fact;
        debug!("{} effect hit of {}", self.fmt(actor), res);
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
        if self.effects[actor].has(Effect::SpeedUpI) {
            fact *= 1.2;
        }
        if self.effects[actor].has(Effect::SpeedDownI) {
            fact *= 0.8;
        }
        if self.effects[actor].has(Effect::SpeedDownII) {
            fact *= 0.6;
        }
        self.get_hero(actor).speed  * fact 
    }

    pub fn get_effect_resistance(&self,actor : InstanceIndex) -> f32 {
        // TODO handle effect resistance buff/debuff
        let mut fact = 1.0;
        debug!("{} base effect resistance of {}", self.fmt(actor),self.get_hero(actor).effect_resistance);
        indent!({
            if self.effects[actor].has(Effect::EffectResistanceDownII) {
                let xfact = 0.5;
                debug!("{} has EffectResistanceDownII -> effect_resistance * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::EffectResistanceDownI) {
                let xfact = 0.75;
                debug!("{} has EffectResistanceDownI -> effect_resistance * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::EffectResistanceUpI) {
                let xfact = 1.25;
                debug!("{} has EffectResistanceUpI -> effect_resistance * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::EffectResistanceUpII) {
                let xfact = 1.5;
                debug!("{} has EffectResistanceUpII -> effect_resistance * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
        });
        let res = self.get_hero(actor).effect_resistance * fact;
        debug!("{} effect resistance of {}", self.fmt(actor), res);
        res
    }

    pub fn get_basic_attack_damage(&self,actor : InstanceIndex) -> f32 {
        if self.has_effect(actor,Effect::RippleII) {
            self.get_attack(actor) * 1.40
        }
        else {
            self.get_attack(actor)
        }
    }

    
    pub fn get_attack_damage(&self,actor : InstanceIndex) -> f32 {
        self.get_attack(actor) 
    }

    pub fn get_defense(&self,actor:InstanceIndex) -> f32 {
        let mut fact = 1.0;
        debug!("{} base defense of {}", self.fmt(actor),self.get_hero(actor).defense);
        indent!({
            if self.effects[actor].has(Effect::DefenseUpI) {
                let xfact = 1.3;
                debug!("{} has DefenseUpI -> defense * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::DefenseUpII) {
                let xfact = 1.6;
                debug!("{} has DefenseUpII -> defense * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::DefenseDownI) {
                let xfact = 0.7;
                debug!("{} has DefenseDownI -> defense * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::DefenseDownII) {
                let xfact = 0.4;
                debug!("{} has DefenseDownII -> defense * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
        });

        let res = self.get_hero(actor).defense *fact;
        debug!("{} defense of {}", self.fmt(actor), res);
        res

    }

    pub fn get_tenacity(&self,actor:InstanceIndex) -> f32 {
        let mut fact = 1.0;
        debug!("{} base tenacity of {}", self.fmt(actor),self.get_hero(actor).tenacity);
        indent!({
            if self.effects[actor].has(Effect::TenacityUpI) {
                let xfact = 1.3;
                debug!("{} has TenacityUpI -> tenacity * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::TenacityUpII) {
                let xfact = 1.6;
                debug!("{} has TenacityUpII -> tenacity * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::TenacityDownI) {
                let xfact = 0.7;
                debug!("{} has TenacityDownI -> tenacity * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::TenacityDownII) {
                let xfact = 0.4;
                debug!("{} has TenacityDownII -> tenacity * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
        });
        let res = self.get_hero(actor).tenacity *fact;
        debug!("{} tenacity of {}", self.fmt(actor), res);
        res
    }

}