use crate::data::effect::is_buff;
use crate::{data::effect::Effect, roll, debug, wave::stat::effect_to_stat, indent};

use super::{ Wave, InstanceIndex};
use rand::thread_rng;
use rand::seq::SliceRandom;

impl<const LEN:usize> Wave<'_,LEN> {
    // TODO this is broken
    pub fn cleanse<F>(&mut self,  actor :InstanceIndex, effect_closure:&F, layers: u32) where F : Fn(Effect) -> bool {
        for (k,v) in self.effects[actor].em.iter_mut() {
            if effect_closure(k) {
                // drop `layers` randomly of v
                if v.len() > layers as usize {
                    let mut rng = rand::thread_rng();
                    //let mut v = v.clone();
                    v.shuffle(&mut rng);
                    for _i in 0..layers {
                        v.pop();
                    }
                }
                else {
                    // empty v
                    v.clear();
                }
            }
        }
        self.effects[actor].remove_empty();
    }

    pub fn cleanse_team<F>(&mut self, actor : InstanceIndex, effect_closure: &F ,layers:u32) where F : Fn(Effect) -> bool {
        for i in self.get_ally_indices(actor) {
            self.cleanse(i,effect_closure,layers);
        }
    }

    pub fn get_number_of_buff_layers(&self, actor : InstanceIndex) -> u32 {
        let mut n = 0;
        for (k,v) in self.effects[actor].em.iter() {
            if is_buff(k) {
                n += v.len() as u32;
            }
        }
        n
    }

    pub fn remove_all_buffs_single(&mut self, actor : InstanceIndex, target : InstanceIndex) {
        for (k,v) in self.effects[target].em.iter_mut() {
            if is_buff(k) {
                // empty v
                v.clear();
            }
        }
        self.effects[target].remove_empty();
    }
}