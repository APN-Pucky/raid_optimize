use crate::{
    data::{effect::Effect, skill::Skill, subskill::Trigger},
    debug, indent, roll, warn,
    wave::{
        for_ally_skill, for_skill,
        heroes::{
            agatha::aristocratic_style::AristocraticStyle, nordak::holy_creature::HolyCreature,
        },
        stat::Stat,
    },
};

use super::{InstanceIndex, Wave};

impl Wave<'_> {
    pub fn shield(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        shield_value: f32,
        shield_turns: u32,
    ) {
        if self.is_dead(target) {
            warn!(
                "{} is dead, cannot shield [{},{}]",
                self.name(target),
                self.health[target],
                self.health[target] > 0.0
            );
            return;
        }
        for_ally_skill!(self, target, Skill::HolyCreature(_), i, {
            self.inflict_single(target, i, Effect::DivineDust, 1.0, 999);
            if i == target {
                self.inflict_single(target, i, Effect::DivineDust, 1.0, 999);
            }
        });
        for i in self.get_enemies_indices(target) {
            for_skill!(
                self,
                i,
                Skill::AristocraticStyle(AristocraticStyle {
                    steal_shield_and_heal_chance
                }),
                {
                    if roll(steal_shield_and_heal_chance) {
                        debug!(
                            "{} transfers shield from {}",
                            self.name(i),
                            self.name(target)
                        );
                        self.shield(i, i, shield_value, shield_turns);
                        return;
                    }
                }
            );
        }
        debug!(
            "{} shields {} for {} for {}",
            self.name(actor),
            self.name(target),
            shield_value,
            shield_turns
        );
        self.add_stat(actor, Stat::Shielded, shield_value);
        self.shields[target].push((shield_value, shield_turns, actor));
        self.on_trigger(actor, Trigger::Shielding);
        self.on_trigger(target, Trigger::Shielded);
    }

    pub fn shield_single(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        shield_value: f32,
        shield_turns: u32,
    ) {
        self.shield(actor, target, shield_value, shield_turns);
    }

    pub fn shield_ally_team(&mut self, actor: InstanceIndex, shield_value: f32, shield_turns: u32) {
        if shield_turns == 0 {
            return;
        }
        debug!(
            "{} shields ally team for {} for {}",
            self.name(actor),
            shield_value,
            shield_turns
        );
        indent!({
            for i in self.get_ally_indices(actor) {
                self.shield_single(actor, i, shield_value, shield_turns);
            }
        })
    }

    pub fn shield_reduce(&mut self, actor: InstanceIndex) {
        let shield = &mut self.shields[actor];
        let mut i = 0;
        while i < shield.len() {
            shield[i].1 -= 1;
            if shield[i].1 == 0 {
                shield.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn shields_reduce(&mut self) {
        // reduce all shields
        for i in 0..self.shields.len() {
            self.shield_reduce(i);
        }
    }

    pub fn clear_shield(&mut self, actor: InstanceIndex, target: InstanceIndex) {
        debug!(
            "{} clears shield from {}",
            self.name(actor),
            self.name(target)
        );
        self.shields[target].clear();
    }

    pub fn steal_shield(&mut self, actor: InstanceIndex, target: InstanceIndex) {
        debug!(
            "{} steals shield from {}",
            self.name(actor),
            self.name(target)
        );
        let val = self.shields[target].clone();
        self.shields[actor].extend(val);
        self.clear_shield(actor, target);
    }

    pub fn shield_subtract(&mut self, actor: InstanceIndex, var: f32) {
        self.add_stat(actor, Stat::ShieldBlocked, var);
        let shield = &mut self.shields[actor];
        let mut value = var;
        let i = 0;
        while i < shield.len() {
            if shield[i].0 > value {
                shield[i].0 -= value;
                return;
            } else {
                value -= shield[i].0;
                shield.remove(i);
            }
        }
    }

    pub fn get_divine_shield(&self, actor: InstanceIndex) -> f32 {
        let mut ret = 0.;
        if self.has_effect(actor, Effect::DivineShield) {
            for_ally_skill!(
                self,
                actor,
                Skill::HolyCreature(HolyCreature {
                    divine_dust_increase_shield,
                    divine_shield_max_health_ratio,
                    ..
                }),
                i,
                { ret = divine_shield_max_health_ratio * self.get_max_health(i) }
            )
        }
        ret
    }

    pub fn get_shield_effect(&self, actor: InstanceIndex) -> f32 {
        let mut shield_effect: f32 = 1.0;

        shield_effect
    }

    pub fn get_shield(&self, actor: InstanceIndex) -> f32 {
        let mut shield: f32 = 0.0;
        for (s, _, _) in self.shields[actor].iter() {
            shield += s;
        }
        // shield effect increase
        for_ally_skill!(
            self,
            actor,
            Skill::HolyCreature(HolyCreature {
                divine_dust_increase_shield,
                divine_shield_max_health_ratio,
                ..
            }),
            i,
            {
                shield *= 1.0
                    + divine_dust_increase_shield
                        * self.effects[actor].get(Effect::DivineDust) as f32;
            }
        );
        shield
    }

    pub fn shield_loose(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        mut damage: f32,
    ) -> f32 {
        let divine_shield = self.get_divine_shield(actor);
        if damage > divine_shield {
            // TODO this is not really correct, but it's a start
            // We should track fractional divine shields
            damage -= divine_shield;
            self.effects[target].clear_single(Effect::DivineShield);
            self.add_stat(target, Stat::DivineShieldBlocked, divine_shield);
        }
        let current_shield = self.get_shield(target);
        if current_shield > damage {
            debug!("{} looses {} shield", self.name(actor), damage);
            self.shield_subtract(target, damage);
            0.0
        } else if current_shield == 0.0 {
            damage
        } else {
            // damage > shield
            debug!("{} looses all {} shield", self.name(actor), current_shield);
            self.add_stat(target, Stat::ShieldBlocked, current_shield);
            self.shields[target] = Vec::new();
            self.on_destroys_shield_alahan(actor, target);
            damage - current_shield
        }
    }
}
