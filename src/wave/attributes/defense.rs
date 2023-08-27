
use crate::{debug, indent, hero::{effect::Effect, skill::{Skill, get_targets, execute_skill}, Hero, instance::Instance, faction::Faction}};

use super::{InstanceIndex, Wave};

impl<const LEN:usize> Wave<'_,LEN> {

    pub fn get_defense(&self,actor:InstanceIndex) -> f32 {
        let mut fact = 1.0;
        debug!("{} base defense of {}", self.fmt(actor),self.get_hero(actor).defense);
        indent!({
            if self.heroes[actor].faction == Faction::Foresters {
                let xfact = self.team_bonds[self.teams[actor]][Faction::Foresters];
                debug!("{} has {} bond with Foresters -> defense * {}", self.fmt(actor), xfact, xfact);
                fact *= xfact;
            }
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
}