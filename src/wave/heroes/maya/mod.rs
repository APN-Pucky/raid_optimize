use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill}, effect::{Effect, is_dot}}};

use self::force_of_mercy::ForceOfMercy;



pub mod light_of_purifying;
pub mod sacred_light;
pub mod force_of_mercy;


impl Wave<'_> {
    pub fn on_damage_dealt_maya(&mut self, _actor : InstanceIndex, target :InstanceIndex,_dmg : f32,_skill : &Skill) {
        for i in self.get_ally_indices(target) {
            if let [ Skill::ForceOfMercy (ForceOfMercy{ max_hp_restore_ratio, .. }) ,  ..] = self.heroes[i].skills[..] {
                self.restore_single(i, i, max_hp_restore_ratio* self.get_max_health(i));
            }
        }

    }

    pub fn on_fatal_damage_maya(&mut self, actor:InstanceIndex) {
        if let [  Skill::ForceOfMercy(s) , ..] = self.heroes[actor].skills[..] {
            // Only once per wave
            if !self.has_effect(actor, Effect::ForceOfMercy) {
                self.restore_single(actor, actor, self.get_max_health(actor));
                self.inflict_single(actor,actor, Effect::ForceOfMercy, 1.0, 999);
            }
        }
    }
}

#[cfg(test)]
mod tests;