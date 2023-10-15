use crate::{
    data::{effect::Effect, skill::Skill},
    wave::{InstanceIndex, Wave},
};

use self::{commendation::Commendation, spirit_call::SpiritCall};

pub mod commendation;
pub mod detach;
pub mod spirit_call;
pub mod spirit_fountain;

impl Wave<'_> {
    pub fn on_damage_dealt_alahan(
        &mut self,
        actor: InstanceIndex,
        _target: InstanceIndex,
        dmg: f32,
        skill: &Skill,
    ) {
        if let Skill::SpiritCall(SpiritCall {
            attack_damage_ratio: _,
            restore_hp_damage_ratio,
            remove_all_buffs: _,
            heal_lowest_ally,
            increase_hp: _,
            ..
        }) = skill
        {
            self.heal(actor, actor, dmg * restore_hp_damage_ratio);
            if *heal_lowest_ally {
                let lowest = self.get_lowest_health_ally(actor);
                self.heal(actor, lowest, dmg * restore_hp_damage_ratio);
            }
        }
    }
    pub fn on_destroys_shield_alahan(&mut self, actor: InstanceIndex, _target: InstanceIndex) {
        for i in self.get_ally_indices(actor) {
            if self.is_alive(i) {
                for (si, s) in self.heroes[i].skills.iter().enumerate() {
                    if let Skill::Commendation(Commendation {
                        max_hp_restore_ratio,
                        attack_up_turns,
                        ..
                    }) = s
                    {
                        if self.is_ready(i, si) {
                            self.restore_single(
                                i,
                                actor,
                                self.get_max_health(i) * max_hp_restore_ratio,
                            );
                            self.inflict_single(
                                i,
                                actor,
                                Effect::AttackUpII,
                                1.0,
                                *attack_up_turns,
                            );
                            self.cooldown(actor, si)
                        }
                    }
                }
            }
        }
    }
}

use super::test_hero;
test_hero!(Alahan);
