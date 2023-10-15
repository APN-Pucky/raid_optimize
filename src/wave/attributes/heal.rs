use crate::{
    data::{faction::Faction, skill::Skill},
    debug, indent,
    wave::heroes::maya::force_of_mercy::ForceOfMercy,
};

use super::{InstanceIndex, Wave};

impl Wave<'_> {
    pub fn get_revive_extra_hp_ratio(&self, actor: InstanceIndex) -> f32 {
        let mut add = 1.0;
        let base = 0.0;
        debug!(
            "{} base revive_extra_hp_ratio of {}",
            self.name(actor),
            base
        );
        indent!({
            if self.get_faction(actor) == Faction::TheForgotten {
                let nfact = self.get_bond(actor, Faction::TheForgotten);
                let r = 0.15 + (nfact - 1.0) * 0.05;
                debug!(
                    "{} has {} bond with HolyLightParliament -> revive_extra_hp_ratio + {}",
                    self.fmt(actor),
                    nfact,
                    r
                );
                add += r;
            }
        });
        let res = base + add;
        debug!("{} revive_extra_hp_ratio of {}", self.name(actor), res);
        res
    }

    pub fn get_healing_effect(&self, actor: InstanceIndex) -> f32 {
        let mut fact = 1.0;
        debug!(
            "{} base healing effect of {}",
            self.name(actor),
            self.get_hero(actor).healing_effect
        );
        indent!({
            if self.get_faction(actor) == Faction::HolyLightParliament {
                let xfact = self.get_bond(actor, Faction::HolyLightParliament);
                debug!(
                    "{} has {} bond with HolyLightParliament -> healing_effect * {}",
                    self.name(actor),
                    xfact,
                    xfact
                );
                fact *= xfact;
            }
            if let [Skill::ForceOfMercy(ForceOfMercy { healing_effect, .. }), ..] =
                self.heroes[actor].skills[..]
            {
                if self.health[actor] < 0.5 * self.get_max_health(actor) {
                    debug!(
                        "{} has ForceOfMercy -> healing_effect * {}",
                        self.name(actor),
                        healing_effect
                    );
                    fact *= healing_effect;
                }
            }
        });
        let res = self.get_hero(actor).healing_effect * fact;
        if fact != 1.0 {
            debug!("{} healing effect of {}", self.name(actor), res);
        }
        res
    }

    pub fn get_healed_effect(&self, actor: InstanceIndex) -> f32 {
        // TODO handle healing buff/debuff
        let mut fact = 1.0;
        let base_healed = 1.0;
        debug!("{} base healed effect of {}", self.name(actor), base_healed);
        indent!({
            if self.get_faction(actor) == Faction::HolyLightParliament {
                let xfact = self.get_bond(actor, Faction::HolyLightParliament);
                debug!(
                    "{} has {} bond with HolyLightParliament -> healed_effect * {}",
                    self.name(actor),
                    xfact,
                    xfact
                );
                fact *= xfact;
            }
        });
        let res = base_healed * fact;
        if fact != 1.0 {
            debug!("{} healed effect of {}", self.name(actor), res);
        }
        res
    }
}
