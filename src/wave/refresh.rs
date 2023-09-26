use crate::data::effect::{is_buff, self};
use crate::{data::effect::Effect, roll, debug, wave::stat::effect_to_stat, indent};

use super::{ Wave, InstanceIndex};
use rand::thread_rng;
use rand::seq::SliceRandom;

impl Wave<'_> {

    pub fn refresh_enemy_team<F>(&mut self, actor : InstanceIndex, effect_closure: &F) where F : Fn(Effect) -> bool {
        for i in self.get_enemies_indices(actor) {
            self.refresh(actor,i,effect_closure);
        }
    }

    pub fn refresh<F>(&mut self, actor : InstanceIndex, target: InstanceIndex,effect_closure: &F) where F : Fn(Effect) -> bool {
        debug!("{} refreshes {}'s", self.name(actor), self.name(target) );
        indent!({
            for (e,v) in self.effects[target].em.iter_mut() {
                if effect_closure(e) {
                    for (cur,start,_ir) in v.iter_mut() {
                        *cur = *start;
                    }
                }
            }
        });
    }

}