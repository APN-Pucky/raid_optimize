use crate::{debug, indent, hero::{effect::Effect, skill::{Skill, get_targets, execute_skill}, Hero, instance::Instance, faction::Faction}};

use super::{InstanceIndex, Wave};


impl<const LEN:usize> Wave<'_,LEN> {

    pub fn count_faction(&self, actor : InstanceIndex, faction : Faction) -> usize {
        let mut count = 0;
        for i in self.get_ally_indices(actor) { 
            if self.heroes[i].faction == faction {
                count+=1;
            }
        }
        count
    }
}