use crate::{wave::{Wave, InstanceIndex}, data::{skill::Skill, effect::Effect, passive::Passive, }, };

impl<'a,const LEN:usize> Wave<'a,LEN> {
    pub fn on_inflicted_margarita(&mut self, target: InstanceIndex, effect : Effect, ) {
        match effect {
            Effect::Blade => {
                for p in &self.heroes[target].passives {
                    match p {
                        Passive::CounterattackCommand { crit_damage_turns, attack_damage_ratio, blades,.. } => {
                            let n = self.effects[target].get(Effect::Blade);
                            if n >= *blades {
                                self.attack_enemy_team(target, self.get_attack_damage(target) * attack_damage_ratio, &Skill::BasicAttack { cooldown: 0, basic_attack: true, attack_damage_ratio: 1.0 });
                                self.inflict_ally_team(target, Effect::CritDamageUpI, 1.0, *crit_damage_turns);
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