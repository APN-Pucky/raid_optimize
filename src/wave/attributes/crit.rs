
use crate::{debug, indent, data::{effect::Effect, skill::{Skill, get_targets, },  instance::Instance, faction::Faction}};

use super::{InstanceIndex, Wave};

impl<const LEN:usize> Wave<'_,LEN> {

    pub fn get_crit_rate(&self, actor: InstanceIndex) -> f32 {
        // TODO handle healing buff/debuff
        let mut fact = 1.0;
        debug!("{} base crit rate of {}", self.name(actor),self.get_hero(actor).crit_rate);
        indent!({
            if self.get_faction(actor) == Faction::DragonTribe {
                let n = self.bonds_counter[actor] as f32;
                let xfact = self.get_bond(actor,Faction::DragonTribe);
                let r = 1.0 +xfact * n;
                debug!("{} has {}*{} bond with DragonTribe -> crit_rate * {}", self.name(actor), n,xfact, r);
                fact *= r;
            }
            if self.effects[actor].has(Effect::CritRateUpI) {
                let xfact = 1.25;
                debug!("{} has CritRateUpI -> crit_rate * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::CritRateUpII) {
                let xfact = 1.5;
                debug!("{} has CritRateUpII -> crit_rate * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::CritRateDownI) {
                let xfact = 0.75;
                debug!("{} has CritRateDownI -> crit_rate * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::CritRateDownII) {
                let xfact = 0.5;
                debug!("{} has CritRateDownII -> crit_rate * {}", self.name(actor), xfact);
                fact *= xfact;
            }

        });
        let ret = self.get_hero(actor).crit_rate *fact;
        if fact != 1.0 {
            debug!("{} crit rate of {}", self.name(actor), ret);
        }
        ret
    }

    pub fn get_crit_damage(&self, actor: InstanceIndex) -> f32 {
        // TODO handle healing buff/debuff
        let mut fact = 1.0;
        debug!("{} base crit damage of {}", self.name(actor),self.get_hero(actor).crit_damage);
        indent!({
            if self.effects[actor].has(Effect::ScarletSakura) {
                let xfact = 1.0 + 0.04 * self.effects[actor].get(Effect::ScarletSakura) as f32;
                debug!("{} has ScarletSakura -> crit_damage * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::CritDamageUpI) {
                let xfact = 1.25;
                debug!("{} has CritDamageUpI -> crit_damage * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::CritDamageUpII) {
                let xfact = 1.5;
                debug!("{} has CritDamageUpII -> crit_damage * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::CritDamageDownI) {
                let xfact = 0.75;
                debug!("{} has CritDamageDownI -> crit_damage * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::CritDamageDownII) {
                let xfact = 0.5;
                debug!("{} has CritDamageDownII -> crit_damage * {}", self.name(actor), xfact);
                fact *= xfact;
            }

        });
        let ret = self.get_hero(actor).crit_damage *fact;
        if fact != 1.0 {
            debug!("{} crit damage of {}", self.name(actor), ret);
        }
        ret
    }
}