
use crate::{debug, indent, data::{effect::Effect, skill::{Skill, get_selection, },  instance::Instance, faction::Faction}};

use super::{InstanceIndex, Wave};

impl Wave<'_> {

    pub fn get_defense(&self,actor:InstanceIndex) -> f32 {
        let mut fact = 1.0;
        debug!("{} base defense of {}", self.name(actor),self.get_hero(actor).defense);
        indent!({
            if self.heroes[actor].faction == Faction::Foresters {
                let xfact = self.team_bonds[self.teams[actor]][Faction::Foresters];
                debug!("{} has {} bond with Foresters -> defense * {}", self.name(actor), xfact, xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::DefenseUpI) {
                let xfact = 1.3;
                debug!("{} has DefenseUpI -> defense * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::DefenseUpII) {
                let xfact = 1.6;
                debug!("{} has DefenseUpII -> defense * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::DefenseDownI) {
                let xfact = 0.7;
                debug!("{} has DefenseDownI -> defense * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::DefenseDownII) {
                let xfact = 0.4;
                debug!("{} has DefenseDownII -> defense * {}", self.name(actor), xfact);
                fact *= xfact;
            }
        });

        let res = self.get_hero(actor).defense *fact;
        debug!("{} defense of {}", self.name(actor), res);
        res

    }
}