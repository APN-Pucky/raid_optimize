use crate::{wave::{Wave, InstanceIndex, attributes::attack}, data::{skill::Skill, effect::{Effect, is_buff, is_attribute_debuff}, passive::Passive}, indent, debug};


impl<'a,const LEN:usize> Wave<'a,LEN> {
    pub fn execute_skill_alahan(&mut self,  skill : &Skill, actor :InstanceIndex, target :InstanceIndex, ) {
        let attacker = actor;
        let defender = target;
        match skill {
            Skill::SpiritCall{ cooldown, basic_attack, attack_damage_ratio, restore_hp_damage_ratio, remove_all_buffs, heal_lowest_ally, increase_hp } => {
                let n = self.get_number_of_buff_layers(defender).min(5);
                if *remove_all_buffs {
                    self.remove_all_buffs_single(actor, target);
                }
                else {
                    panic!("Not implemented");
                }
                self.attack_single(actor, target, self.get_attack_damage(actor) * attack_damage_ratio,
                 &Skill::SpiritCall { 
                    cooldown: *cooldown, 
                    basic_attack: *basic_attack, 
                    attack_damage_ratio: *attack_damage_ratio, 
                    restore_hp_damage_ratio:*restore_hp_damage_ratio + 0.2 * n as f32, 
                    remove_all_buffs:*remove_all_buffs, 
                    heal_lowest_ally:*heal_lowest_ally, 
                    increase_hp:*increase_hp 
                });

            },
            Skill::SpiritFountain { cooldown, basic_attack, heal_turns, cleanse_attrubute_debuffs } => {
                let a = self.get_highest_health_percentage_ally(actor);
                let hp = self.health[a] / self.get_max_health(a);
                for a in self.get_ally_indices(actor) {
                    self.heal(actor,a, self.get_max_health(a)* hp - self.health[a]);
                }
                if *cleanse_attrubute_debuffs {
                    self.cleanse_team(actor, &is_attribute_debuff, 999)
                }
                self.inflict_ally_team(actor, Effect::Heal, 1.0, *heal_turns);
            },
            Skill::Detach { cooldown, basic_attack, attack_damage_ratio, stun_chance, stun_turns, steal_shield, shield_max_hp_ratio,shield_turns } => {
                // remove all enemies shields
                for e in self.get_enemies_indices(actor) {
                    if *steal_shield {
                        self.steal_shield(actor, e);
                    }
                    else {
                        self.clear_shield(actor, e);
                    }
                }
                self.attack_enemy_team(actor, attack_damage_ratio* self.get_attack_damage(actor), skill);
                self.inflict_enemy_team(actor, Effect::Stun, *stun_chance, *stun_turns);
                self.shield_ally_team(actor, self.get_max_health(actor) *shield_max_hp_ratio, *shield_turns);

            },
            _ => {}

    
        }
    }
    pub fn on_damage_dealt_alahan(&mut self, actor : InstanceIndex, target :InstanceIndex,dmg : f32,skill : &Skill) {
        match skill {
            Skill::SpiritCall { cooldown, basic_attack, attack_damage_ratio, restore_hp_damage_ratio, remove_all_buffs, heal_lowest_ally, increase_hp } => {
                self.heal(actor,actor,dmg * restore_hp_damage_ratio);
                if *heal_lowest_ally {
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
                match self.heroes[i].passives[..] {
                        [Passive::Commendation { max_hp_ratio, attack_up_turns } ,..] => {
                            self.restore_single(i,actor, self.get_max_health(i)*max_hp_ratio);
                            self.inflict_single(i,actor, Effect::AttackUpII, 1.0, attack_up_turns);
                        },
                        _  => {},
                }
            }
        }

    }
}