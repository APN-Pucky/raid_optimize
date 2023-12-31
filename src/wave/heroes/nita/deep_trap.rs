use derive_macro::PassiveSkill;

use crate::{
    data::{effect::Effect, skill::Skill},
    debug, roll,
    wave::{
        for_skill, has_skill,
        heroes::{BasicAttack, Cooldown, PassiveSkill},
        InstanceIndex, Wave,
    },
};

#[derive(PassiveSkill, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct DeepTrap {
    pub pursue_and_attack_limit: u32,
    pub poison_cleansed_attack_chance: f32,
}

impl Default for DeepTrap {
    fn default() -> Self {
        Self {
            pursue_and_attack_limit: 5,
            poison_cleansed_attack_chance: 0.7,
        }
    }
}

impl Wave<'_> {
    pub fn nita_convert_poison_to_heal(&mut self, actor: InstanceIndex) {
        if has_skill!(self, actor, Skill::DeepTrap(_)) {
            debug!("DeepTrap converts poison to Heal");
            for v in self.effects[actor].clone_single(Effect::Poison) {
                self.inflict_single(actor, actor, Effect::Heal, 1.0, v.1);
            }
            self.effects[actor].clear_single(Effect::Poison);
            self.effects[actor].clear_single(Effect::_DeepTrapCounter);
        }
    }

    pub fn nita_on_poison_cleanse(&mut self, cleansed: InstanceIndex) {
        for i in self.get_enemies_indices(cleansed) {
            for_skill!(
                self,
                i,
                Skill::DeepTrap(DeepTrap {
                    pursue_and_attack_limit,
                    poison_cleansed_attack_chance,
                }),
                {
                    if self.effects[i].get(Effect::_DeepTrapCounter) < pursue_and_attack_limit {
                        self.inflict_single(i, i, Effect::_DeepTrapCounter, 1.0, 1);
                        if roll(poison_cleansed_attack_chance) {
                            // TODO is this just a attack or a BasicAttack skill (i.e. Curse)
                            self.attack_single(
                                i,
                                cleansed,
                                self.get_attack_damage(i),
                                &Skill::BasicAttack(BasicAttack {
                                    cooldown: 0,
                                    attack_damage_ratio: 1.0,
                                }),
                            );
                        }
                    }
                }
            );
        }
    }
}
