use enum_map::EnumMap;
use rand::Rng;

use crate::{debug, wave::stat::Stat, indent, data::{faction::Faction, mark::Mark, skill::{Skill, is_basic_attack, NONE_SKILL, BASIC_ATTACK}, effect::Effect}};

use super::{Wave, InstanceIndex};

impl Wave<'_> {
    pub fn control(&mut self, actor : InstanceIndex, target:InstanceIndex) {
        debug!("{} control {}", self.name(actor), self.name(target));
        indent!({
            if let Some([Skill { data: SkillData::SoulRing {..}, .. } ,..])  =  self.heroes[target].skills[..] {
                debug!("{} has SoulRing -> immune to control", self.name(target));
                return;
            }
            if self.has_effect(target, Effect::ControlImmunity) {
                debug!("{} has ControlImmunity -> immune to control", self.name(target));
                return;
            }
        });
    }
}