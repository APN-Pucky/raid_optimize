use crate::data::effect::{Effect, EffectFilter};

use super::{InstanceIndex, Wave};

use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

impl Wave<'_> {
    // TODO this is broken
    pub fn cleanse(&mut self, actor: InstanceIndex, effect_closure: EffectFilter, layers: u32) {
        for k in Effect::iter().filter(effect_closure) {
            let v = self.effects[actor].mut_single(k);
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
