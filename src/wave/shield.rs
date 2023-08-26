use crate::{wave::stat::Stat, debug, indent, hero::instance::Instance};

use super::{Wave, InstanceIndex};

impl<const LEN:usize> Wave<'_,LEN> {
    pub fn shield(&mut self, actor : InstanceIndex, target : InstanceIndex,shield_value:f32, shield_turns:u32) {
        debug!("{} shields {} for {} for {}", actor, target, shield_value, shield_turns);
        self.add_stat(actor, Stat::Shielded, shield_value);
        self.shields[target].push((shield_value, shield_turns));
    }

    pub fn shield_single(&mut self, actor : InstanceIndex, target : InstanceIndex,shield_value:f32, shield_turns:u32) {
        self.shield(actor, target, shield_value, shield_turns);
    }

    pub fn shield_ally_team(&mut self, actor : InstanceIndex,shield_value:f32, shield_turns:u32) {
        if shield_turns == 0 {
            return;
        }
        debug!("{} shields ally team for {} for {}", actor, shield_value, shield_turns);
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
            }
            else {
                i += 1;
            }
        }
    }

    pub fn shields_reduce(&mut self){
        // reduce all shields
        for i in 0..self.shields.len() {
            self.shield_reduce(i);
        }
    }

    pub fn shield_subtract(&mut self,actor : InstanceIndex, var:f32) {
        self.add_stat(actor,Stat::ShieldBlocked, var);
        let shield = &mut self.shields[actor];
        let mut value = var;
        let i = 0;
        while i < shield.len() {
            if shield[i].0 > value {
                shield[i].0 -= value;
                return;
            }
            else {
                value -= shield[i].0;
                shield.remove(i);
            }
        }
    }

    pub fn get_shield(&self, actor : InstanceIndex) -> f32 {
        let mut shield: f32 = 0.0;
        for (s,_) in self.shields[actor].iter() {
            shield += s;
        }
        shield
    }


    pub fn shield_loose(&mut self,actor : InstanceIndex, damage: f32) -> f32 {
        let current_shield = self.get_shield(actor);
        if current_shield > damage {
            debug!("{} looses {} shield", actor, damage);
            self.shield_subtract(actor,damage);
            0.0
        }
        else if current_shield == 0.0 {
            damage
        }
        else { // damage > shield
            debug!("{} looses all {} shield", actor, current_shield);
            self.add_stat(actor,Stat::ShieldBlocked, current_shield);
            self.shields[actor] = Vec::new();
            damage - current_shield
        }
    }

}