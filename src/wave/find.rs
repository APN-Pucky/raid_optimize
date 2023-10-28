use ordered_float::OrderedFloat;

use super::{InstanceIndex, Wave};

impl Wave<'_> {
    pub fn find_highest_attack_alive_enemy(&self, actor: InstanceIndex) -> Option<InstanceIndex> {
        self.get_enemies_indices_iter(actor)
            .filter(|&a| self.is_alive(a))
            .max_by_key(|&a| OrderedFloat(self.get_attack(a)))
    }
    pub fn find_highest_attack_alive_ally(&self, actor: InstanceIndex) -> Option<InstanceIndex> {
        self.get_ally_indices_iter(actor)
            .filter(|&a| self.is_alive(a))
            .max_by_key(|&a| OrderedFloat(self.get_attack(a)))
    }
}
