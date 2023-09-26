use crate::{wave::{Wave, InstanceIndex, attributes::attack}, data::{skill::{Skill, SkillData}, effect::{Effect, is_buff, is_attribute_debuff}}, indent, debug};


impl Wave<'_> {
    pub fn execute_skill_alahan(&mut self,  skill : &Skill, actor :InstanceIndex, target :InstanceIndex, ) {
        let attacker = actor;
        let defender = target;
        match skill.data {
            SkillData::SpiritCall{ attack_damage_ratio, restore_hp_damage_ratio, remove_all_buffs, heal_lowest_ally, increase_hp } => {
                let n = self.get_number_of_buff_layers(defender).min(5);
                if remove_all_buffs {
                    self.remove_all_buffs_single(actor, target);
                }
                else {
                    panic!("Not implemented");
                }
                self.attack_single(actor, target, self.get_attack_damage(actor) * attack_damage_ratio,
                 &Skill{
                    cooldown : skill.cooldown,
                    typ : skill.typ,
                    select: skill.select,
                    data : SkillData::SpiritCall { 
                        attack_damage_ratio: attack_damage_ratio, 
                        restore_hp_damage_ratio:restore_hp_damage_ratio + 0.2 * n as f32, 
                        remove_all_buffs:remove_all_buffs, 
                        heal_lowest_ally:heal_lowest_ally, 
                        increase_hp:increase_hp 
                    }
                });

            },
            SkillData::SpiritFountain { heal_turns, cleanse_attribute_debuffs } => {
                self.restore_to_highest_ally_health_percentage(actor);
                if cleanse_attribute_debuffs {
                    self.cleanse_team(actor, &is_attribute_debuff, 999)
                }
                self.inflict_ally_team(actor, Effect::Heal, 1.0, heal_turns);
            },
            SkillData::Detach { attack_damage_ratio, stun_chance, stun_turns, steal_shield, shield_max_hp_ratio,shield_turns } => {
                // remove all enemies shields
                for e in self.get_enemies_indices(actor) {
                    if steal_shield {
                        self.steal_shield(actor, e);
                    }
                    else {
                        self.clear_shield(actor, e);
                    }
                }
                self.attack_enemy_team(actor, attack_damage_ratio* self.get_attack_damage(actor), skill);
                self.inflict_enemy_team(actor, Effect::Stun, stun_chance, stun_turns);
                self.shield_ally_team(actor, self.get_max_health(actor) *shield_max_hp_ratio, shield_turns);

            },
            _ => {}

    
        }
    }
    pub fn on_damage_dealt_alahan(&mut self, actor : InstanceIndex, target :InstanceIndex,dmg : f32,skill : &Skill) {
        match skill.data {
            SkillData::SpiritCall { attack_damage_ratio, restore_hp_damage_ratio, remove_all_buffs, heal_lowest_ally, increase_hp } => {
                self.heal(actor,actor,dmg * restore_hp_damage_ratio);
                if heal_lowest_ally {
                    let lowest = self.get_lowest_health_ally(actor);
                    self.heal(actor,lowest,dmg * restore_hp_damage_ratio);
                }
            },
            _ => {}
        }
    }
    pub fn on_destroys_shield_alahan(&mut self, actor : InstanceIndex, target :InstanceIndex) {
        for i in self.get_ally_indices(actor) {
            if self.is_alive(i) {
                for (si , s) in  self.heroes[i].skills.iter().enumerate() {
                    match s {
                            Skill { data: SkillData::Commendation{max_hp_restore_ratio, attack_up_turns  }, .. } => {
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