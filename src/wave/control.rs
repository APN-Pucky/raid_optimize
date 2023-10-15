use enum_map::EnumMap;
use rand::Rng;

use crate::{
    data::{
        effect::Effect,
        faction::Faction,
        mark::Mark,
        skill::{is_basic_attack, Skill, BASIC_ATTACK, NONE_SKILL},
    },
    debug, indent,
    wave::stat::Stat,
};

use super::{InstanceIndex, Wave};

impl Wave<'_> {
    pub fn control(&mut self, actor: InstanceIndex, target: InstanceIndex) {
        debug!("{} control {}", self.name(actor), self.name(target));
        indent!({
            if let Some(
                [Skill {
                    data: SkillData::SoulRing { .. },
                    ..
                }, ..],
            ) = self.heroes[target].skills[..]
            {
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
