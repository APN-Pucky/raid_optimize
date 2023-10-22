use enum_map::EnumMap;
use rand::Rng;

use crate::{
    data::{
        effect::Effect,
        faction::Faction,
        mark::Mark,
        skill::{is_basic_attack, Skill},
    },
    debug, indent,
    wave::{has_skill, stat::Stat},
};

use super::{InstanceIndex, Wave};

impl Wave<'_> {
    pub fn control(&mut self, actor: InstanceIndex, target: InstanceIndex) {
        debug!("{} control {}", self.name(actor), self.name(target));
        indent!({
            if has_skill!(self, target, Skill::SoulRing(_)) {
                debug!("{} has SoulRing -> immune to control", self.name(target));
                return;
            }
            if self.has_effect(target, Effect::ControlImmunity) {
                debug!(
                    "{} has ControlImmunity -> immune to control",
                    self.name(target)
                );
                return;
            }
        });
    }
}
