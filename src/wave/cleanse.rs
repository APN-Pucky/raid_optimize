use crate::data::effect::EffectFilter;

use super::{InstanceIndex, Wave};

use rand::seq::SliceRandom;

impl Wave<'_> {
    // TODO this is broken
    pub fn cleanse(&mut self, actor: InstanceIndex, effect_closure: EffectFilter, layers: u32) {
        for (k, v) in self.effects[actor].em.iter_mut() {
            if effect_closure(&k) {
                // drop `layers` randomly of v
                if v.len() > layers as usize {
                    let mut rng = rand::thread_rng();
                    //let mut v = v.clone();
                    v.shuffle(&mut rng);
                    for _i in 0..layers {
                        v.pop();
                    }
                } else {
                    // empty v
                    v.clear();
                }
            }
        }
        self.effects[actor].remove_empty();
    }

    pub fn cleanse_team(
        &mut self,
        actor: InstanceIndex,
        effect_closure: EffectFilter,
        layers: u32,
    ) {
        for i in self.get_ally_indices(actor) {
            self.cleanse(i, effect_closure, layers);
        }
    }
}
