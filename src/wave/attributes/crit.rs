
use crate::{debug, indent, hero::{effect::Effect, skill::{Skill, get_targets, execute_skill}, Hero, instance::Instance, faction::Faction}};

use super::{InstanceIndex, Wave};

impl<const LEN:usize> Wave<'_,LEN> {

    pub fn get_crit_rate(&self, actor: InstanceIndex) -> f32 {
        // TODO handle healing buff/debuff
        let mut fact = 1.0;
        debug!("{} base crit rate of {}", self.fmt(actor),self.get_hero(actor).crit_rate);
        indent!({
            if self.effects[actor].has(Effect::CritRateUpI) {
                let xfact = 1.25;
                debug!("{} has CritRateUpI -> crit_rate * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::CritRateUpII) {
                let xfact = 1.5;
                debug!("{} has CritRateUpII -> crit_rate * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::CritRateDownI) {
                let xfact = 0.75;
                debug!("{} has CritRateDownI -> crit_rate * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::CritRateDownII) {
                let xfact = 0.5;
                debug!("{} has CritRateDownII -> crit_rate * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }

        });
        let ret = self.get_hero(actor).crit_rate *fact;
        debug!("{} crit rate of {}", self.fmt(actor), ret);
        ret
    }

    pub fn get_crit_damage(&self, actor: InstanceIndex) -> f32 {
        // TODO handle healing buff/debuff
        let mut fact = 1.0;
        debug!("{} base crit damage of {}", self.fmt(actor),self.get_hero(actor).crit_damage);
        indent!({
            if self.effects[actor].has(Effect::CritDamageUpI) {
                let xfact = 1.25;
                debug!("{} has CritDamageUpI -> crit_damage * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::CritDamageUpII) {
                let xfact = 1.5;
                debug!("{} has CritDamageUpII -> crit_damage * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::CritDamageDownI) {
                let xfact = 0.75;
                debug!("{} has CritDamageDownI -> crit_damage * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::CritDamageDownII) {
                let xfact = 0.5;
                debug!("{} has CritDamageDownII -> crit_damage * {}", self.fmt(actor), xfact);
                fact *= xfact;
            }

        });
        let ret = self.get_hero(actor).crit_damage *fact;
        debug!("{} crit damage of {}", self.fmt(actor), ret);
        ret
    }
}