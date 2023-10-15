use crate::{debug, indent, wave::stat::Stat};

use super::{InstanceIndex, Wave};

impl Wave<'_> {
    pub fn shield(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        shield_value: f32,
        shield_turns: u32,
    ) {
        debug!(
            "{} shields {} for {} for {}",
            self.name(actor),
            self.name(target),
            shield_value,
            shield_turns
        );
        self.add_stat(actor, Stat::Shielded, shield_value);
        self.shields[target].push((shield_value, shield_turns, actor));
    }

    pub fn shield_single(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        shield_value: f32,
        shield_turns: u32,
    ) {
        self.shield(actor, target, shield_value, shield_turns);
    }

    pub fn shield_ally_team(&mut self, actor: InstanceIndex, shield_value: f32, shield_turns: u32) {
        if shield_turns == 0 {
            return;
        }
        debug!(
            "{} shields ally team for {} for {}",
            self.name(actor),
            shield_value,
            shield_turns
        );
        indent!({
            for i in self.get_ally_indices(actor) {
                self.shield_single(actor, i, shield_value, shield_turns);
            }
        })
    }

    pub fn shield_reduce(&mut self, actor: InstanceIndex) {
        let shield = &mut self.shields[actor];
        let mut i = 0;
        while i < shield.len() {
            shield[i].1 -= 1;
            if shield[i].1 == 0 {
                shield.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn shields_reduce(&mut self) {
        // reduce all shields
        for i in 0..self.shields.len() {
            self.shield_reduce(i);
        }
    }

    pub fn clear_shield(&mut self, actor: InstanceIndex, target: InstanceIndex) {
        debug!(
            "{} clears shield from {}",
            self.name(actor),
            self.name(target)
        );
        self.shields[target].clear();
    }

    pub fn steal_shield(&mut self, actor: InstanceIndex, target: InstanceIndex) {
        debug!(
            "{} steals shield from {}",
            self.name(actor),
            self.name(target)
        );
        let val = self.shields[target].clone();
        self.shields[actor].extend(val);
        self.clear_shield(actor, target);
    }

    pub fn shield_subtract(&mut self, actor: InstanceIndex, var: f32) {
        self.add_stat(actor, Stat::ShieldBlocked, var);
        let shield = &mut self.shields[actor];
        let mut value = var;
        let i = 0;
        while i < shield.len() {
            if shield[i].0 > value {
                shield[i].0 -= value;
                return;
            } else {
                value -= shield[i].0;
                shield.remove(i);
            }
        }
    }

    pub fn get_shield(&self, actor: InstanceIndex) -> f32 {
        let mut shield: f32 = 0.0;
        for (s, _, _) in self.shields[actor].iter() {
            shield += s;
        }
        shield
    }

    pub fn shield_loose(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        damage: f32,
    ) -> f32 {
        let current_shield = self.get_shield(target);
        if current_shield > damage {
            debug!("{} looses {} shield", self.name(actor), damage);
            self.shield_subtract(target, damage);
            0.0
        } else if current_shield == 0.0 {
            damage
        } else {
            // damage > shield
            debug!("{} looses all {} shield", self.name(actor), current_shield);
            self.add_stat(target, Stat::ShieldBlocked, current_shield);
            self.shields[target] = Vec::new();
            self.on_destroys_shield_alahan(actor, target);
            damage - current_shield
        }
    }
}
