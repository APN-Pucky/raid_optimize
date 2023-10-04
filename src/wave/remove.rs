use crate::data::effect::{is_buff};
use crate::{data::effect::Effect, debug, indent};

use super::{ Wave, InstanceIndex};



impl Wave<'_> {

    pub fn remove_all_buffs_single(&mut self, actor : InstanceIndex, target : InstanceIndex) {
        debug!("{} removes all buffs from {}", self.name(actor), self.name(target) );
        indent!({
            if self.effects[target].has(Effect::BlockRemoval) {
                debug!("{} has block_removal, no buffs removed", self.name(target));
                self.effects[target].remove_layer(Effect::BlockRemoval);
                return;
            }
            for (k,v) in self.effects[target].em.iter_mut() {
                if is_buff(k) {
                    // empty v
                    v.clear();
                }
            }
            self.effects[target].remove_empty();
        });
    }
}