use crate::{
    data::{effect::Effect, faction::Faction, skill::Skill, subskill::Trigger},
    debug, indent, roll,
    wave::{has_skill, stat::effect_to_stat},
};

use super::{InstanceIndex, Wave};

impl Wave<'_> {
    fn inflict(&mut self, actor: InstanceIndex, target: InstanceIndex, effect: Effect, turns: u32) {
        match effect {
            Effect::HPBurning => self.inflict_hp_burning(actor, target, turns),
            Effect::Bleed => self.inflict_bleed(actor, target, turns),
            _ => {
                self.inflict_any(actor, target, effect, turns);
            }
        }
    }

    fn inflict_any(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        effect: Effect,
        turns: u32,
    ) -> bool {
        debug!(
            "{} inflicts {} for {} turns on {}",
            self.name(actor),
            effect,
            turns,
            self.name(target)
        );
        indent!({
            if self.has_effect(target, Effect::BlockBuff) && effect.is_buff() {
                debug!("{} has BlockBuff, {} is blocked", self.name(target), effect);
                return false;
            }
            if self.effects[target].get(effect) >= effect.get_max() {
                debug!("{} already has max {}", self.name(target), effect);
                return false;
            }
            //if self.heroes[target].skills.iter().any(|x| match x {Skill::AristocraticStyle(_) => true, _ => false}) {
            if has_skill!(self, target, Skill::AristocraticStyle(_)) {
                if effect == Effect::SevereWound || effect == Effect::InferiorSevereWound {
                    debug!(
                        "{} has AristocraticStyle dodges {}",
                        self.name(target),
                        effect
                    );
                    return false;
                }
            }
            let mut turns: u32 = turns;
            self.on_inflict_dakota(actor, target, effect, &mut turns);

            self.add_stat(actor, effect_to_stat(effect), turns as f32);
            self.effects[target].push(effect, turns, actor);
            self.on_inflicted_margarita(target, effect);
            self.on_trigger(actor, Trigger::Inflicting);
            self.on_trigger(target, Trigger::Inflicted);
            if effect == Effect::Silence {
                self.on_trigger(target, Trigger::InflictedSilence);
            }
            if effect.is_control() {
                self.on_trigger(target, Trigger::InflictedControl);
            }
            if actor == target
                && self.get_faction(actor) == Faction::DoomLegion
                && effect.is_buff()
                && self.bonds_counter[actor] < 5
            {
                self.bonds_counter[actor] += 1;
            }
            true
        });
        true
    }

    fn inflict_hp_burning(&mut self, actor: InstanceIndex, target: InstanceIndex, turns: u32) {
        self.inflict_any(actor, target, Effect::HPBurning, turns);
    }

    fn inflict_bleed(&mut self, actor: InstanceIndex, target: InstanceIndex, turns: u32) {
        if self.inflict_any(actor, target, Effect::Bleed, turns) {
            let dmg_vec = vec![
                0.00, 0.14, 0.18, 0.22, 0.26, 0.30, 0.30, 0.30, 0.30, 0.30, 0.30,
            ];
            let bleed_dmg = self.get_attack_damage(actor)
                * dmg_vec[(self.effects[target].get(Effect::Bleed)) as usize];
            self.damage_bleed(actor, target, bleed_dmg);
        }
    }

    pub fn inflict_single(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        effect: Effect,
        chance: f32,
        turns: u32,
    ) {
        indent!({
            if !effect.is_debuff() {
                self.inflict_buff_single(actor, target, effect, turns);
            } else {
                self.inflict_debuff_single(actor, target, effect, chance, turns);
            }
        });
    }
    pub fn inflict_buff_single(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        effect: Effect,
        turns: u32,
    ) {
        // no rolling here
        self.inflict(actor, target, effect, turns);
    }
    pub fn inflict_debuff_single(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        effect: Effect,
        chance: f32,
        turns: u32,
    ) {
        debug!(
            "{} inflicts {} for {} turns on {} with {}% change",
            self.name(actor),
            effect,
            turns,
            self.name(target),
            chance * 100.
        );
        let eh = self.get_effect_hit(actor);
        let er = self.get_effect_resistance(target);
        let mchance = chance * (1.0 + eh - er);

        if roll(mchance) {
            self.inflict(actor, target, effect, turns);
        } else {
            debug!(
                "{} misses {} inflict on {}",
                self.name(actor),
                effect,
                self.name(target)
            );
        }
    }

    pub fn inflict_enemy_team(
        &mut self,
        actor: InstanceIndex,
        effect: Effect,
        chance: f32,
        turns: u32,
    ) {
        if turns == 0 {
            return;
        }
        debug!(
            "{} inflicts {} for {} on enemy team",
            self.name(actor),
            effect,
            turns
        );
        indent!({
            for i in self.get_enemies_indices(actor) {
                self.inflict_single(actor, i, effect, chance, turns);
            }
        })
    }

    pub fn inflict_ally_team(
        &mut self,
        actor: InstanceIndex,
        effect: Effect,
        chance: f32,
        turns: u32,
    ) {
        if turns == 0 {
            return;
        }
        debug!(
            "{} inflicts {} for {} on ally team",
            self.name(actor),
            effect,
            turns
        );
        indent!({
            for i in self.get_ally_indices(actor) {
                self.inflict_single(actor, i, effect, chance, turns);
            }
        })
    }
}
