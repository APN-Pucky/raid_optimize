use crate::{debug, wave::stat::Stat, indent, warn};

use super::{Wave, InstanceIndex};

impl<const LEN:usize> Wave<'_,LEN> {
    pub fn heal(&mut self,actor :InstanceIndex, health:f32) {
        if self.is_dead(actor) {
            warn!("{} is dead, cannot heal [{},{}]", self.name(actor),self.health[actor],self.health[actor]> 0.0);
            return;
        }
        let healing_effect = self.get_healing_effect(actor);
        //let health = self.health[actor]; 
        let heal = health * (1.+healing_effect); // TODO handle rounding
        let new_health = self.get_max_health(actor).min(self.health[actor] + heal);
        debug!("{} heals {} health (healing_effect: {})", self.name(actor), heal, healing_effect);
        self.add_stat(actor,Stat::HealthHealed, new_health- self.health[actor] );
        self.health[actor] = new_health;
    }

    pub fn restore(&mut self, actor : InstanceIndex, target: InstanceIndex,health:f32) {
        if self.is_dead(target) {
            return;
        }
        debug!("{} restores {} for {}", self.name(actor), self.name(target), health);
        self.add_stat(actor, Stat::HealthRestored, health);
        self.heal(target, health)
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