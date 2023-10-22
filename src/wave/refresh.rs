use strum::IntoEnumIterator;

use crate::{
    data::effect::{Effect, EffectFilter},
    debug, indent,
};

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
            for k in Effect::iter().filter(effect_closure) {
                let v = self.effects[actor].mut_single(k);
                for (cur, start, _ir) in v.iter_mut() {
                    *cur = *start;
                }
            }
        });
    }
}
