use crate::{
    data::{effect::Effect, skill::Skill},
    wave::{for_skill, has_skill, InstanceIndex, Wave},
};

use self::force_of_mercy::ForceOfMercy;

pub mod force_of_mercy;
pub mod light_of_purifying;
pub mod sacred_light;

impl Wave<'_> {
    pub fn on_damage_dealt_maya(
        &mut self,
        _actor: InstanceIndex,
        target: InstanceIndex,
        _dmg: f32,
        _skill: &Skill,
    ) {
        for i in self.get_ally_indices(target) {
            for_skill!(
                self,
                i,
                Skill::ForceOfMercy(ForceOfMercy {
                    max_hp_restore_ratio,
                    ..
                }),
                {
                    self.restore_single(i, i, max_hp_restore_ratio * self.get_max_health(i));
                }
            );
            //for p in &self.heroes[i].skills {
            //if let Skill::ForceOfMercy(ForceOfMercy {
            //    max_hp_restore_ratio,
            //    ..
            //}) = *p
            //{
            //    self.restore_single(i, i, max_hp_restore_ratio * self.get_max_health(i));
            //}}
        }
    }

    pub fn on_fatal_damage_maya(&mut self, actor: InstanceIndex) {
        if has_skill!(self, actor, Skill::ForceOfMercy(_)) {
            // Only once per wave
            if !self.has_effect(actor, Effect::ForceOfMercy) {
                self.restore_single(actor, actor, self.get_max_health(actor));
                self.inflict_single(actor, actor, Effect::ForceOfMercy, 1.0, 999);
            }
        }
    }
}

#[cfg(test)]
mod tests;
