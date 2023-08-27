use crate::{debug, wave::stat::Stat, indent, warn};

use super::{Wave, InstanceIndex};

impl<const LEN:usize> Wave<'_,LEN> {
    pub fn heal(&mut self,actor : InstanceIndex, target :InstanceIndex, health:f32) {
        if self.is_dead(target) {
            warn!("{} is dead, cannot heal [{},{}]", self.name(target),self.health[target],self.health[target]> 0.0);
            return;
        }
        let healing_effect = self.get_healing_effect(actor);
        let healed_effect = self.get_healed_effect(target);
        let heal = health * (healing_effect+healed_effect); // TODO handle rounding
        let new_health = self.get_max_health(target).min(self.health[target] + heal);
        debug!("{} heals {} health (healing_effect: {}, healed_effect: {})", self.name(target), heal, healing_effect, healed_effect);
        self.add_stat(target,Stat::HealthHealed, new_health- self.health[target] );
        self.health[target] = new_health;
    }

    pub fn restore(&mut self, actor : InstanceIndex, target: InstanceIndex,health:f32) {
        if self.is_dead(target) {
            return;
        }
        debug!("{} restores {} for {}", self.name(actor), self.name(target), health);
        indent!({
            self.add_stat(actor, Stat::HealthRestored, health);
            self.heal(actor,target, health)
        });
    }

    pub fn restore_single(&mut self, actor : InstanceIndex, target: InstanceIndex,health:f32) {
        self.restore(actor, target, health);
    }
    
    pub fn restore_ally_team(&mut self, actor : InstanceIndex, restore_hp: f32) {
        debug!("{} restores own team for {}", self.name(actor), restore_hp);
        indent!({
            for i in self.get_ally_indices(actor) {
                self.restore_single(actor, i, restore_hp);
            }
        })
    }

    pub fn restore_max_hp_ratio_own_team(&mut self, actor : InstanceIndex, restore_max_hp_ratio: f32) {
        debug!("{} restores own team by {} of their max_hp ", self.name(actor), restore_max_hp_ratio);
        indent!({
            for i in self.get_ally_indices(actor) {
                self.restore_single(actor, i, self.get_max_health(i)*restore_max_hp_ratio);
            }
        })
    }
}