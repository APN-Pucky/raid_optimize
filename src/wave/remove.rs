use crate::data::effect::{is_buff, self};
use crate::{data::effect::Effect, roll, debug, wave::stat::effect_to_stat, indent};

use super::{ Wave, InstanceIndex};
use rand::thread_rng;
use rand::seq::SliceRandom;

impl<const LEN:usize> Wave<'_,LEN> {

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