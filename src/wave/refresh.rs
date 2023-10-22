use crate::{data::effect::EffectFilter, debug, indent};

use super::{InstanceIndex, Wave};

impl Wave<'_> {
    pub fn refresh_enemy_team(&mut self, actor: InstanceIndex, effect_closure: EffectFilter) {
        for i in self.get_enemies_indices(actor) {
            self.refresh(actor, i, effect_closure);
        }
    }

    pub fn refresh(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        effect_closure: EffectFilter,
    ) {
        debug!("{} refreshes {}'s", self.name(actor), self.name(target));
        indent!({
            for (e, v) in self.effects[target].em.iter_mut() {
                if effect_closure(&e) {
                    for (cur, start, _ir) in v.iter_mut() {
                        *cur = *start;
                    }
                }
            }
        });
    }
}
