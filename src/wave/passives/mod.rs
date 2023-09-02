use crate::data::passive::Passive;

use super::{InstanceIndex, Wave};

pub mod sharp_instinct;

impl<const LEN:usize> Wave<'_,LEN> {
    pub fn has_passive(&self, actor:InstanceIndex, passive: Passive) -> bool {
        self.heroes[actor].passives.iter().any(|p| *p == passive)
    }
}