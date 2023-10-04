use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill, SkillData, BASIC_ATTACK}, effect::Effect, }, };

impl Wave<'_> {
    pub fn on_inflicted_margarita(&mut self, target: InstanceIndex, effect : Effect, ) {
        match effect {
            Effect::Blade => {
                for p in &self.heroes[target].skills {
                    match p.data {
                        SkillData::CounterattackCommand { crit_damage_turns, attack_damage_ratio, blades,.. } => {
                            let n = self.effects[target].get(Effect::Blade);
                            if n >= blades {
                                self.attack_enemy_team(target, self.get_attack_damage(target) * attack_damage_ratio, &BASIC_ATTACK);
                                self.inflict_ally_team(target, Effect::CritDamageUpI, 1.0, crit_damage_turns);
                                // clear blades
                                self.effects[target].clear_single(Effect::Blade);
                            }
                        },
                        _ => {}
                    }
                }
            },
            _ => {}
        }
    }
}