use crate::{wave::{Wave, InstanceIndex, heroes::dakota::soul_surge::SoulSurge}, data::{skill::{Skill}, effect::Effect}, debug, warn};

use self::soul_ring::SoulRing;

pub mod soul_seal;
pub mod soul_surge;
pub mod soul_ring;

impl Wave<'_> {

    pub fn on_attacked_dakota(&mut self, attacker : InstanceIndex, attacked:InstanceIndex) {
        match self.heroes[attacked].skills[..] {
            [Skill::SoulRing(SoulRing{effect_res_down_chance, effect_res_down_turns }),..] => {
                self.inflict_single(attacked, attacker, Effect::RosePoison, 1.0, 999);
                self.inflict_single(attacked, attacker, Effect::EffectResistanceDownII, effect_res_down_chance,effect_res_down_turns);
            },
            _ => {}
        }
    }

    pub fn on_inflict_dakota(&mut self, _actor : InstanceIndex, target:InstanceIndex, effect : Effect, turns :&mut u32) {
        if self.has_effect(target,Effect::ToxicSwamp) && effect == Effect::Poison {
            if let Some(inflictor) = self.effects[target].get_last_inflictor(Effect::ToxicSwamp) {
                if self.is_alive(inflictor) {
                    debug!("{} has ToxicSwamp, Poison prolonged by one turn", self.name(target));
                    // increase turns by one
                    *turns = *turns+1;
                    match self.heroes[inflictor].skills[..] {
                        [Skill::SoulSurge (SoulSurge{rose_poison_chance,..}) , ..] => {
                            self.inflict_single(inflictor, target, Effect::RosePoison, rose_poison_chance, 999)
                        }
                        _ => {}
                    }
                }
            }
            else {
                warn!("No inflictor for ToxicSwamp on {}", self.name(target));
            }
        }
    }
}



use super::test_hero;
test_hero!(Dakota);