use crate::{
    data::effect::{Effect, EffectFilter},
    debug, indent,
};

use super::{InstanceIndex, Wave};

impl Wave<'_> {
    // difference to cleanse is that this removes all buffs, not just some
    pub fn remove_effect_single(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        effect_closure: EffectFilter,
    ) -> i32 {
        let mut i = 0;
        debug!(
            "{} removes buffs from {}",
            self.name(actor),
            self.name(target)
        );
        indent!({
            if self.effects[target].has(Effect::BlockRemoval) {
                debug!("{} has block_removal, no buffs removed", self.name(target));
                self.effects[target].remove_layer(Effect::BlockRemoval);
                return;
            }
            for (k, v) in self.effects[target].em.iter_mut() {
                if effect_closure(&k) {
                    // empty v
                    v.clear();
                    i += 1;
                }
            }
            self.effects[target].remove_empty();
        });
        i
    }

    pub fn remove_all_buffs_single(&mut self, actor: InstanceIndex, target: InstanceIndex) -> i32 {
        debug!(
            "{} removes all buffs from {}",
            self.name(actor),
            self.name(target)
        );
        self.remove_effect_single(actor, target, Effect::is_buff)
    }
}
