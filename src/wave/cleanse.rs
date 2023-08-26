use crate::{hero::effect::Effect, roll, debug, wave::stat::effect_to_stat, indent};

use super::{ Wave, InstanceIndex};

impl Wave<'_> {
    // TODO this is broken
    pub fn cleanse<F>(&mut self, effect_closure:&F, layers: u32) where F : Fn(Effect) -> bool {
        for (k,v) in self.effects.em.iter_mut() {
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
        self.effects.remove_empty();
    }

    pub fn cleanse_team<F>(&mut self, actor : &InstanceRef, effect_closure: &F ,layers:u32) where F : Fn(Effect) -> bool {
        if actor.team {
            self.allies.iter_mut().for_each(|a| a.cleanse(effect_closure,layers));
        }
        else {
            self.enemies.iter_mut().for_each(|a| a.cleanse(effect_closure,layers));
        }
    }
}