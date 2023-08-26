use crate::{wave::stat::Stat, debug, indent, hero::{instance::Instance, effect::Effect}};

use super::{Wave, InstanceIndex};

impl<const LEN:usize> Wave<'_,LEN> {
    pub fn has_effect(&self, actor: InstanceIndex, effect : Effect) -> bool {
        self.effects[actor].has(effect)
    }

    pub fn effect_reduce(&mut self, actor: InstanceIndex) {
        self.effects[actor].reduce();
    }
}