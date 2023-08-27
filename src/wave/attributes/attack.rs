
use crate::{debug, indent, hero::{effect::Effect, skill::{Skill, get_targets, execute_skill}, Hero, instance::Instance, faction::Faction}};

use super::{InstanceIndex, Wave};

impl<const LEN:usize> Wave<'_,LEN> {


    pub fn get_skill_damage_ratio(&self,actor : InstanceIndex) -> f32 {
        let mut fact  =1.0;
        //let attack = self.get_attack(actor);
        indent!({
            if self.has_effect(actor, Effect::FactionHiddenWaveSkill) {
                let xfact = self.effects[actor].get(Effect::FactionHiddenWaveSkill).min(2) as f32;
                let n = self.get_bond(actor, Faction::HiddenWave);
                let r = xfact * n;
                debug!("{} has FactionHiddenWaveSkill -> skill damage * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }

        });
        let res = fact;
        debug!("{} skill damage ratio of {}", self.fmt(actor), res);
        res
    }

    pub fn get_basic_attack_damage_ratio(&self,actor : InstanceIndex) -> f32 {
        let mut fact  =1.0;
        //let attack = self.get_attack(actor);
        indent!({
            if self.has_effect(actor, Effect::FactionHiddenWaveAttack) {
                let xfact = self.effects[actor].get(Effect::FactionHiddenWaveAttack).min(2) as f32;
                let n = self.get_bond(actor, Faction::HiddenWave);
                let r = xfact * n;
                debug!("{} has FactionHiddenWaveAttack -> basic attack damage * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
            if self.has_effect(actor, Effect::RippleII) {
                let xfact = 1.40;
                debug!("{} has RippleII -> basic attack damage * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }

        });
        let res = fact;
        debug!("{} basic attack damage ratio of {}", self.fmt(actor), res);
        res
    }

    pub fn get_attack(&self,actor : InstanceIndex) -> f32 {
        let mut fact = 1.0;
        debug!("{} base attack of {}", self.fmt(actor),self.get_hero(actor).attack);
        indent!({
            if self.heroes[actor].faction == Faction::Foresters {
                let xfact = self.get_bond(actor,Faction::Foresters);
                debug!("{} has {} bond with Foresters -> attack * {}", self.fmt(actor), xfact, xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::AttackUpI) {
                let xfact = 1.25;
                debug!("{} has AttackUpI -> attack * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::AttackUpII) {
                let xfact = 1.5;
                debug!("{} has AttackUpII -> attack * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::AttackDownI) {
                let xfact = 0.75;
                debug!("{} has AttackDownI -> attack * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::AttackDownII) {
                let xfact = 0.5;
                debug!("{} has AttackDownII -> attack * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
        });
        let ret = self.get_hero(actor).attack * fact;
        debug!("{} attack of {}", self.fmt(actor), ret);
        ret
    }

    pub fn get_piercing(&self, actor : InstanceIndex) -> f32 {
        self.get_hero(actor).piercing
    }
}