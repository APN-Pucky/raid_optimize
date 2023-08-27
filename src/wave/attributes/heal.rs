
use crate::{debug, indent, hero::{effect::Effect, skill::{Skill, get_targets, execute_skill}, Hero, instance::Instance, faction::Faction}};

use super::{InstanceIndex, Wave};

impl<const LEN:usize> Wave<'_,LEN> {

    pub fn get_revive_extra_hp_ratio(&self, actor:InstanceIndex) -> f32 {
        let mut add= 1.0;
        let base = 0.0;
        debug!("{} base revive_extra_hp_ratio of {}", self.fmt(actor),base);
        indent!({
            if self.get_faction(actor) == Faction::TheForgotten{
                let nfact = self.get_bond(actor,Faction::TheForgotten);
                let r = 0.15 + (nfact-1.0)*0.05;
                debug!("{} has {} bond with HolyLightParliament -> revive_extra_hp_ratio + {}", self.fmt(actor), nfact, r);
                add += r;
            }        
        });
        let res = base + add;
        debug!( "{} revive_extra_hp_ratio of {}", self.fmt(actor), res);
        res
    }

    pub fn get_healing_effect(&self, actor: InstanceIndex) -> f32 {
        // TODO handle healing buff/debuff
        let mut fact = 1.0;
        debug!("{} base healing effect of {}", self.fmt(actor),self.get_hero(actor).healing_effect);
        indent!({
            if self.get_faction(actor) == Faction::HolyLightParliament {
                let xfact = self.get_bond(actor,Faction::HolyLightParliament);
                debug!("{} has {} bond with HolyLightParliament -> healing_effect * {}", self.fmt(actor), xfact, xfact);
                fact *= xfact;
            }        
        });
        let res = self.get_hero(actor).healing_effect* fact;
        debug!( "{} healing effect of {}", self.fmt(actor), res);
        res
    }

    pub fn get_healed_effect(&self, actor: InstanceIndex) -> f32 {
        // TODO handle healing buff/debuff
        let mut fact = 1.0;
        let base_healed = 1.0;
        debug!("{} base healed effect of {}", self.fmt(actor),base_healed);
        indent!({
            if self.get_faction(actor) == Faction::HolyLightParliament {
                let xfact = self.get_bond(actor,Faction::HolyLightParliament);
                debug!("{} has {} bond with HolyLightParliament -> healed_effect * {}", self.fmt(actor), xfact, xfact);
                fact *= xfact;
            }        
        });
        let res = base_healed* fact;
        debug!( "{} healing effect of {}", self.fmt(actor), res);
        res
    }
}