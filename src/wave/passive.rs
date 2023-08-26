use crate::hero::passive::Passive;

use super::{InstanceIndex, Wave};

impl<const LEN:usize> Wave<'_,LEN> {
    pub fn has_passive(&self, actor:InstanceIndex, passive: Passive) -> bool {
        self.heroes[actor].passives.iter().any(|p| *p == passive)
    }
}