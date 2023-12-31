use std::cmp;

use ordered_float::OrderedFloat;

use crate::{
    data::{effect::Effect, faction::Faction, hero::Hero, mark::Mark, skill::Skill},
    debug, indent,
    wave::{
        for_ally_skill, for_skill,
        heroes::{nordak::holy_creature::HolyCreature, paulin::prompt_action::PromptAction},
    },
};

use super::{InstanceIndex, Wave};

pub mod attack;
pub mod crit;
pub mod defense;
pub mod heal;
pub mod mastery;

impl Wave<'_> {
    pub fn get_turn_meter_reduction_reduction(&self, actor: InstanceIndex) -> f32 {
        let base = 0.0;
        if self.get_faction(actor) == Faction::TheForgotten {
            let nfact = self.get_bond(actor, Faction::TheForgotten);
            if nfact > 0.0 {
                let fact = 0.05 + (nfact - 1.0) * 0.025;
                debug!(
                    "{} has {} bond with TheForgotten -> turn_meter_reduction_reduction + {}",
                    self.fmt(actor),
                    nfact,
                    fact
                );
                return base + fact;
            }
            base
        } else {
            base
        }
    }

    pub fn get_faction(&self, actor: InstanceIndex) -> Faction {
        self.heroes[actor].faction
    }

    pub fn get_mark(&self, actor: InstanceIndex) -> Mark {
        self.heroes[actor].mark
    }

    pub fn get_leech(&self, actor: InstanceIndex, target: InstanceIndex) -> f32 {
        let mut fact = 1.0;
        debug!(
            "{} base leech of {}",
            self.name(actor),
            self.get_hero(actor).leech
        );
        indent!({
            if self.get_faction(actor) == Faction::SunsetSages {
                let xfact = self.get_bond(actor, Faction::SunsetSages);
                debug!(
                    "{} has {} bond with SunsetSages -> leech * {}",
                    self.name(actor),
                    xfact,
                    xfact
                );
                fact *= xfact;
            }
            let n = self.effects[target].get(Effect::CountessKiss);
            fact *= 1.0 + 0.02 * n as f32;
        });
        if fact != 1.0 {
            debug!(
                "{} leech of {}",
                self.name(actor),
                self.get_hero(actor).leech * fact
            );
        }
        self.heroes[actor].leech * fact
    }

    pub fn get_damage_reflect(&self, actor: InstanceIndex) -> f32 {
        self.heroes[actor].damage_reflection
    }

    pub fn get_effect_hit(&self, actor: InstanceIndex) -> f32 {
        // TODO handle effect hit buff/debuff
        let mut fact = 1.0;
        debug!(
            "{} base effect hit of {}",
            self.name(actor),
            self.get_hero(actor).effect_hit
        );
        indent!({
            if self.heroes[actor].faction == Faction::WizardsEye {
                let xfact = self.get_bond(actor, Faction::WizardsEye);
                debug!(
                    "{} has {} bond with WizardsEye -> effect_hit * {}",
                    self.name(actor),
                    xfact,
                    xfact
                );
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::EffectHitUpI) {
                let xfact = 1.25;
                debug!(
                    "{} has EffectHitUpI -> effect_hit * {}",
                    self.name(actor),
                    xfact
                );
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::EffectHitUpII) {
                let xfact = 1.5;
                debug!(
                    "{} has EffectHitUpII -> effect_hit * {}",
                    self.name(actor),
                    xfact
                );
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::EffectHitDownI) {
                let xfact = 0.75;
                debug!(
                    "{} has EffectHitDownI -> effect_hit * {}",
                    self.name(actor),
                    xfact
                );
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::EffectHitDownII) {
                let xfact = 0.5;
                debug!(
                    "{} has EffectHitDownII -> effect_hit * {}",
                    self.name(actor),
                    xfact
                );
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::FlowingRainbow) {
                let n = self.effects[actor].get(Effect::FlowingRainbow);
                let xfact = 1. - 0.04 * n as f32;
                debug!(
                    "{} has FlowingRainbow {} -> effect_hit * {}",
                    self.name(actor),
                    n,
                    xfact
                );
                fact *= xfact;
            }
        });
        let res = self.get_hero(actor).effect_hit * fact;
        if fact != 1.0 {
            debug!("{} effect hit of {}", self.name(actor), res);
        }
        res
    }

    pub fn is_alive(&self, actor: InstanceIndex) -> bool {
        self.health[actor] > 0.0
    }
    pub fn is_dead(&self, actor: InstanceIndex) -> bool {
        !self.is_alive(actor)
    }

    #[inline]
    pub fn get_hero(&self, actor: InstanceIndex) -> &Hero {
        &self.heroes[actor]
    }

    pub fn get_max_health(&self, actor: InstanceIndex) -> f32 {
        let mut fact = 1.0;
        let mut add = 0.;
        debug!(
            "{} base max health of {}",
            self.name(actor),
            self.get_hero(actor).health
        );
        indent!({
            if self.effects[actor].has(Effect::OverflowingLight) {
                for_ally_skill!(
                    self,
                    actor,
                    Skill::HolyCreature(HolyCreature {
                        overflowing_light_turn_limit,
                        divine_dust_increase_shield,
                        overflowing_light_alive_max_hp_ratio,
                        overflowing_light_dead_max_hp_ratio,
                        divine_shield_max_health_ratio
                    }),
                    i,
                    {
                        let xadd = self.get_hero(i).health
                            * if self.is_alive(i)
                                && self.acted_turns[i] > overflowing_light_turn_limit
                            {
                                overflowing_light_alive_max_hp_ratio
                            } else {
                                overflowing_light_dead_max_hp_ratio
                            };
                        debug!("{} has HolyCreate -> health + {}", self.name(actor), xadd);
                        add += xadd;
                    }
                )
            }
            if self.effects[actor].has(Effect::HPUpI) {
                let xfact = 1.25;
                debug!("{} has HPUpI -> health * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::HPUpII) {
                let xfact = 1.5;
                debug!("{} has HPUpII -> health * {}", self.name(actor), xfact);
                fact *= xfact;
            }
        });
        let res = self.get_hero(actor).health * fact + add;
        if fact != 1.0 {
            debug!("{} speed of {}", self.name(actor), res);
        }
        res
    }

    pub fn get_speed(&self, actor: InstanceIndex) -> f32 {
        let mut fact = 1.0;
        debug!(
            "{} base speed of {}",
            self.name(actor),
            self.get_hero(actor).speed
        );
        indent!({
            if self.effects[actor].has(Effect::SpeedUpI) {
                let xfact = 1.2;
                debug!("{} has SpeedUpI -> speed * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::SpeedUpII) {
                let xfact = 1.4;
                debug!("{} has SpeedUpII -> speed * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::SpeedDownI) {
                let xfact = 0.8;
                debug!("{} has SpeedDownI -> speed * {}", self.name(actor), xfact);
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::SpeedDownII) {
                let xfact = 0.6;
                debug!("{} has SpeedDownII -> speed * {}", self.name(actor), xfact);
                fact *= xfact;
            }
        });
        let res = self.get_hero(actor).speed * fact;
        if fact != 1.0 {
            debug!("{} speed of {}", self.name(actor), res);
        }
        res
    }

    pub fn get_effect_resistance(&self, actor: InstanceIndex) -> f32 {
        // TODO handle effect resistance buff/debuff
        let mut fact = 1.0;
        debug!(
            "{} base effect resistance of {}",
            self.name(actor),
            self.get_hero(actor).effect_resistance
        );
        indent!({
            if self.effects[actor].has(Effect::EffectResistanceDownII) {
                let xfact = 0.5;
                debug!(
                    "{} has EffectResistanceDownII -> effect_resistance * {}",
                    self.name(actor),
                    xfact
                );
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::EffectResistanceDownI) {
                let xfact = 0.75;
                debug!(
                    "{} has EffectResistanceDownI -> effect_resistance * {}",
                    self.name(actor),
                    xfact
                );
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::EffectResistanceUpI) {
                let xfact = 1.25;
                debug!(
                    "{} has EffectResistanceUpI -> effect_resistance * {}",
                    self.name(actor),
                    xfact
                );
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::EffectResistanceUpII) {
                let xfact = 1.5;
                debug!(
                    "{} has EffectResistanceUpII -> effect_resistance * {}",
                    self.name(actor),
                    xfact
                );
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::FlowingRainbow) {
                let n = self.effects[actor].get(Effect::FlowingRainbow);
                let xfact = 1. - 0.04 * n as f32;
                debug!(
                    "{} has FlowingRainbow {} -> effect_resistance * {}",
                    self.name(actor),
                    n,
                    xfact
                );
                fact *= xfact;
            }
        });
        let res = self.get_hero(actor).effect_resistance * fact;
        if fact != 1.0 {
            debug!("{} effect resistance of {}", self.name(actor), res);
        }
        res
    }

    pub fn get_attack_damage(&self, actor: InstanceIndex) -> f32 {
        self.get_attack(actor)
    }

    pub fn get_tenacity(&self, actor: InstanceIndex) -> f32 {
        let mut fact = 1.0;
        debug!(
            "{} base tenacity of {}",
            self.name(actor),
            self.get_hero(actor).tenacity
        );
        indent!({
            if self.effects[actor].has(Effect::TenacityUpI) {
                let xfact = 1.3;
                debug!(
                    "{} has TenacityUpI -> tenacity * {}",
                    self.name(actor),
                    xfact
                );
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::TenacityUpII) {
                let xfact = 1.6;
                debug!(
                    "{} has TenacityUpII -> tenacity * {}",
                    self.name(actor),
                    xfact
                );
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::TenacityDownI) {
                let xfact = 0.7;
                debug!(
                    "{} has TenacityDownI -> tenacity * {}",
                    self.name(actor),
                    xfact
                );
                fact *= xfact;
            }
            if self.effects[actor].has(Effect::TenacityDownII) {
                let xfact = 0.4;
                debug!(
                    "{} has TenacityDownII -> tenacity * {}",
                    self.name(actor),
                    xfact
                );
                fact *= xfact;
            }
            for_skill!(
                self,
                actor,
                Skill::PromptAction(PromptAction {
                    increase_self_turn_meter_ratio,
                    increase_ally_turn_meter_ratio,
                    increase_tenacity_ratio,
                    increase_tenacity_ratio_max,
                    effect_resistance_turns,
                    start_increase_turn_meter_ratio
                }),
                {
                    let xfact = 1.0
                        + (increase_tenacity_ratio * self.get_effect_resistance(actor))
                            .min(increase_tenacity_ratio_max);
                    debug!(
                        "{} has PromptAction -> tenacity * {}",
                        self.name(actor),
                        xfact
                    );
                    fact *= xfact;
                }
            );
        });
        let res = self.get_hero(actor).tenacity * fact;
        if fact != 1.0 {
            debug!("{} tenacity of {}", self.name(actor), res);
        }
        res
    }
}
