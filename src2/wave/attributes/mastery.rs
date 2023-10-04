
use crate::{debug, indent, data::{effect::Effect, skill::{Skill, get_selection, },  instance::Instance, faction::Faction}};

use super::{InstanceIndex, Wave};

impl Wave<'_> {

    pub fn get_mastery(&self,actor : InstanceIndex) -> f32 {
        let mut fact = 1.0;
        debug!("{} base mastery of {}", self.fmt(actor),self.get_hero(actor).mastery);

        indent!({
            if self.get_faction(actor) == Faction::DragonTribe {
                let n = self.bonds_counter[actor] as f32;
                let xfact = self.get_bond(actor,Faction::DragonTribe);
                let r = 1.0 +xfact * n;
                debug!("{} has {}*{} bond with DragonTribe -> mastery * {}", self.fmt(actor), n,xfact, r);
                fact *= r;
            }
        });

        let res = self.heroes[actor].mastery * fact;
        debug!("{} mastery of {}", self.fmt(actor), res);
        res
    }
}