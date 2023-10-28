use crate::{
    data::effect::{self, Effect, EffectFilter},
    debug, indent,
};
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

use super::{InstanceIndex, Wave};

impl Wave<'_> {
    // difference to cleanse is that this removes all buffs, not just some
    pub fn remove_effect_single(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        effect: Effect,
    ) -> u32 {
        let mut i = 0;
        debug!(
            "{} removes effects from {}",
            self.name(actor),
            self.name(target)
        );
        indent!({
            if self.effects[target].has(Effect::BlockRemoval) {
                // TODO should this check if allied or not! since it is negative else
                debug!(
                    "{} has block_removal, no effects removed",
                    self.name(target)
                );
                self.effects[target].remove_layer(Effect::BlockRemoval);
                return i;
            }
            i += self.effects[target].get(effect);
            self.effects[target].clear_single(effect);
            self.effects[target].remove_empty();
            i
        })
    }

    pub fn remove_one_random_effect_filter_single(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        opt_effect_closure: EffectFilter,
    ) -> u32 {
        let mut i = 0;
        let mut effs = Effect::iter()
            .filter(|e| opt_effect_closure(e) && self.effects[target].has(*e))
            .collect::<Vec<_>>();
        if let Some(e) = effs.choose(&mut rand::thread_rng()) {
            self.remove_effect_single(actor, target, *e)
        } else {
            0
        }
    }

    pub fn remove_effect_filter_single(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        opt_effect_closure: EffectFilter,
    ) -> u32 {
        let mut i = 0;
        for e in Effect::iter().filter(|e| opt_effect_closure(e)) {
            i += self.remove_effect_single(actor, target, e);
        }
        i
    }

    pub fn remove_effect_filter_allies(
        &mut self,
        actor: InstanceIndex,
        opt_effect_closure: EffectFilter,
    ) -> u32 {
        let mut i = 0;
        for e in Effect::iter().filter(|e| opt_effect_closure(e)) {
            for target in self.get_ally_indices(actor) {
                i += self.remove_effect_single(actor, target, e);
            }
        }
        i
    }

    pub fn remove_effect_filter_enemies(
        &mut self,
        actor: InstanceIndex,
        opt_effect_closure: EffectFilter,
    ) -> u32 {
        let mut i = 0;
        for e in Effect::iter().filter(|e| opt_effect_closure(e)) {
            for target in self.get_enemies_indices(actor) {
                i += self.remove_effect_single(actor, target, e);
            }
        }
        i
    }

    pub fn remove_all_buffs_single(&mut self, actor: InstanceIndex, target: InstanceIndex) -> u32 {
        debug!(
            "{} removes all buffs from {}",
            self.name(actor),
            self.name(target)
        );
        self.remove_effect_filter_single(actor, target, Effect::is_buff)
    }
}
