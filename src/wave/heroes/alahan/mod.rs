use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill}, effect::{Effect, is_attribute_debuff}}};

use self::{spirit_call::SpiritCall, commendation::Commendation};

pub mod detach;
pub mod spirit_call;
pub mod spirit_fountain;
pub mod commendation;

impl Wave<'_> {
    pub fn on_damage_dealt_alahan(&mut self, actor : InstanceIndex, _target :InstanceIndex,dmg : f32,skill : &Skill) {
        match skill {
            Skill::SpiritCall(SpiritCall { attack_damage_ratio: _, restore_hp_damage_ratio, remove_all_buffs: _, heal_lowest_ally, increase_hp: _,.. }) => {
                self.heal(actor,actor,dmg * restore_hp_damage_ratio);
                if *heal_lowest_ally {
                    let lowest = self.get_lowest_health_ally(actor);
                    self.heal(actor,lowest,dmg * restore_hp_damage_ratio);
                }
            },
            _ => {}
        }
    }
    pub fn on_destroys_shield_alahan(&mut self, actor : InstanceIndex, _target :InstanceIndex) {
        for i in self.get_ally_indices(actor) {
            if self.is_alive(i) {
                for (si , s) in  self.heroes[i].skills.iter().enumerate() {
                    match s {
                            Skill::Commendation(Commendation{max_hp_restore_ratio, attack_up_turns,..  }) => {
                                if self.is_ready(i,si) {
                                    self.restore_single(i,actor, self.get_max_health(i)*max_hp_restore_ratio);
                                    self.inflict_single(i,actor, Effect::AttackUpII, 1.0, *attack_up_turns);
                                    self.cooldown(actor,si)
                                }
                            },
                            _  => {},
                    }
                }
            }
        }

    }
}

use super::test_hero;
test_hero!(Alahan);