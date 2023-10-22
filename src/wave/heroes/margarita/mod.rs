use crate::{
    data::{effect::Effect, skill::Skill},
    wave::{for_skill, InstanceIndex, Wave},
};

use self::counterattack_command::CounterattackCommand;

use super::BasicAttack;

pub mod counterattack_command;

impl Wave<'_> {
    pub fn on_inflicted_margarita(&mut self, target: InstanceIndex, effect: Effect) {
        if let Effect::Blade = effect {
            for_skill!(
                self,
                target,
                Skill::CounterattackCommand(CounterattackCommand {
                    crit_damage_turns,
                    attack_damage_ratio,
                    blades,
                    ..
                }),
                {
                    let n = self.effects[target].get(Effect::Blade);
                    if n >= blades {
                        self.attack_enemy_team(
                            target,
                            self.get_attack_damage(target) * attack_damage_ratio,
                            &Skill::BasicAttack(BasicAttack {
                                cooldown: 0,
                                attack_damage_ratio: 1.0,
                            }),
                        );
                        self.inflict_ally_team(
                            target,
                            Effect::CritDamageUpI,
                            1.0,
                            crit_damage_turns,
                        );
                        // clear blades
                        self.effects[target].clear_single(Effect::Blade);
                    }
                }
            );
        }
    }
}

#[cfg(test)]
mod tests;
