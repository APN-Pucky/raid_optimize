use crate::{
    data::{effect::Effect, skill::Skill},
    debug, warn,
    wave::{for_skill, heroes::dakota::soul_surge::SoulSurge, InstanceIndex, Wave},
};

use self::soul_ring::SoulRing;

pub mod soul_ring;
pub mod soul_seal;
pub mod soul_surge;

impl Wave<'_> {
    pub fn on_attacked_dakota(&mut self, attacker: InstanceIndex, attacked: InstanceIndex) {
        for_skill!(
            self,
            attacked,
            Skill::SoulRing(SoulRing {
                effect_res_down_chance,
                effect_res_down_turns,
            }),
            {
                self.inflict_single(attacked, attacker, Effect::RosePoison, 1.0, 999);
                self.inflict_single(
                    attacked,
                    attacker,
                    Effect::EffectResistanceDownII,
                    effect_res_down_chance,
                    effect_res_down_turns,
                );
            }
        );
    }

    pub fn on_inflict_dakota(
        &mut self,
        _actor: InstanceIndex,
        target: InstanceIndex,
        effect: Effect,
        turns: &mut u32,
    ) {
        if self.has_effect(target, Effect::ToxicSwamp) && effect == Effect::Poison {
            if let Some(inflictor) = self.effects[target].get_last_inflictor(Effect::ToxicSwamp) {
                if self.is_alive(inflictor) {
                    debug!(
                        "{} has ToxicSwamp, Poison prolonged by one turn",
                        self.name(target)
                    );
                    // increase turns by one
                    *turns += 1;
                    for_skill!(
                        self,
                        inflictor,
                        Skill::SoulSurge(SoulSurge {
                            rose_poison_chance,
                            ..
                        }),
                        {
                            self.inflict_single(
                                inflictor,
                                target,
                                Effect::RosePoison,
                                rose_poison_chance,
                                999,
                            )
                        }
                    );
                }
            } else {
                warn!("No inflictor for ToxicSwamp on {}", self.name(target));
            }
        }
    }
}

use super::test_hero;
test_hero!(Dakota);
