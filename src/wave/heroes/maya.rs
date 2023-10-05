use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill}, effect::{Effect, is_dot}}};


use rand::seq::SliceRandom;


impl Wave<'_> {
    pub fn execute_skill_maya(&mut self,  skill : &Skill, actor :InstanceIndex, target :InstanceIndex, ) {
        match skill.data {
            SkillData::LightOfPurifying{ max_hp_restore_ratio, heal_turns, cleanse_dot_layers, heal_allies  } =>{
                let t =  target;
                if self.health[t] == self.get_max_health(t) {
                    self.cleanse(target, &is_dot, cleanse_dot_layers);
                    self.inflict_single(actor, target, Effect::Heal, 1.0, heal_turns);

                }
                self.restore_single(actor, target, max_hp_restore_ratio* self.get_max_health(actor));
                // also heal 2 random allies
                let mut allies = self.get_ally_indices(actor);
                let mut rng = rand::thread_rng();
                allies.shuffle(&mut rng);
                let mut n = 0;
                while n < heal_allies {
                    if let Some(i) = allies.pop() {
                        if i != target {
                            self.restore_single(actor, i, max_hp_restore_ratio* self.get_max_health(actor));
                        }
                    }
                    n += 1;
                }
            },
            SkillData::SacredLight { max_hp_restore_ratio, loose_hp_ratio, consolidation_turns, shield_turns, shield_max_hp_ratio, block_debuff_turns } => {
                self.loose_health(actor, loose_hp_ratio * self.get_max_health(actor));
                // 1st
                for t in self.get_ally_indices(actor) {
                    if self.health[t] == self.get_max_health(t) {
                        self.inflict_single(actor, t, Effect::ConsolidationI, 1.0, consolidation_turns);
                    }
                    self.restore_single(actor, t, max_hp_restore_ratio* self.get_max_health(actor));
                }
                // 2nd
                for t in self.get_ally_indices(actor) {
                    if self.health[t] == self.get_max_health(t) {
                        self.shield_single(actor, t, self.get_max_health(actor) * shield_max_hp_ratio, shield_turns);
                    }
                    self.restore_single(actor, t, max_hp_restore_ratio* self.get_max_health(actor));
                }
                // 3rd
                for t in self.get_ally_indices(actor) {
                    if self.health[t] == self.get_max_health(t) {
                        self.inflict_single(actor, t, Effect::BlockDebuff, 1.0, block_debuff_turns);
                    }
                    self.restore_single(actor, t, max_hp_restore_ratio* self.get_max_health(actor));
                }
            }
            _ => {}
        }
    }

    pub fn on_damage_dealt_maya(&mut self, _actor : InstanceIndex, target :InstanceIndex,_dmg : f32,_skill : &Skill) {
        for i in self.get_ally_indices(target) {
            if let [ Skill { data : SkillData::ForceOfMercy { max_hp_restore_ratio, .. } , .. }, ..] = self.heroes[i].skills[..] {
                self.restore_single(i, i, max_hp_restore_ratio* self.get_max_health(i));
            }
        }

    }

    pub fn on_fatal_damage_maya(&mut self, actor:InstanceIndex) {
        if let [ Skill { data : SkillData::ForceOfMercy { .. } , .. }, ..] = self.heroes[actor].skills[..] {
            // Only once per wave
            if !self.has_effect(actor, Effect::ForceOfMercy) {
                self.restore_single(actor, actor, self.get_max_health(actor));
                self.inflict_single(actor,actor, Effect::ForceOfMercy, 1.0, 999);
            }
        }
    }
}