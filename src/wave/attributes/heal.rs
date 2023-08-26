
use crate::{debug, indent, hero::{effect::Effect, skill::{Skill, get_targets, execute_skill}, Hero, instance::Instance, faction::Faction}};

use super::{InstanceIndex, Wave};

impl<const LEN:usize> Wave<'_,LEN> {

    pub fn get_healing_effect(&self, actor: InstanceIndex) -> f32 {
        // TODO handle healing buff/debuff
        self.get_hero(actor).healing_effect
    }
}