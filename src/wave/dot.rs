use crate::{
    data::{effect::Effect, skill::Skill},
    debug, warn,
};

use super::{heroes::nita::deep_trap::DeepTrap, InstanceIndex, Wave};

impl Wave<'_> {
    /// Check and apply damage from poison
    pub fn dot_poison(&mut self, actor: InstanceIndex) {
        let n = self.effects[actor].get(Effect::Poison);
        if n > 0 {
            if let Some(inflictor) = self.effects[actor].get_last_inflictor(Effect::Poison) {
                let poison_dmg = self.get_attack_damage(inflictor);

                let deep_poison = if self.effects[actor].get(Effect::DeepPoison) > 0 {
                    1.25
                } else {
                    1.0
                };
                let mastery = self.get_mastery(inflictor);

                self.damage_poison(inflictor, actor, poison_dmg * (1.0 + mastery) * deep_poison);
            } else {
                warn!("No inflictor for poison {}", self.name(actor));
            }
        }
    }

    pub fn dot_heal(&mut self, actor: InstanceIndex) {
        // apply heal
        let n = self.effects[actor].get(Effect::Heal);
        if n > 0 {
            if let Some(nn) = self.effects[actor].get_last_inflictor(Effect::Heal) {
                let heal = self.get_max_health(actor) * 0.05 * n as f32;
                self.heal(nn, actor, heal);
            } else {
                warn!("No inflictor for heal on {}", self.name(actor));
            }
        }
    }

    pub fn dot_bleed(&mut self, actor: InstanceIndex) {
        if let [Skill::DeepTrap(DeepTrap { .. }), ..] = self.heroes[actor].skills[..] {
            debug!("DeepTrap garants immunity against DoT");
            return;
        }
        // apply bleed
        let n = self.effects[actor].get(Effect::Bleed);
        if n > 0 {
            if let Some(inflictor) = self.effects[actor].get_last_inflictor(Effect::Bleed) {
                //let nn: u32= b.iter().map(|(n,_)| n).sum();
                let dmg_vec = vec![0.30, 0.50, 0.70, 0.90, 1.05, 1.20, 1.35, 1.45, 1.55, 1.65];
                let bleed_dmg = self.get_attack_damage(inflictor) * dmg_vec[n as usize - 1];
                let mastery = self.get_mastery(inflictor);
                self.damage_bleed(inflictor, actor, bleed_dmg * (1.0 + mastery));
            } else {
                warn!("No inflictor for bleed on {}", self.name(actor));
            }
        }
    }

    pub fn dot_hp_burning(&mut self, actor: InstanceIndex) {
        if let [Skill::DeepTrap(DeepTrap { .. }), ..] = self.heroes[actor].skills[..] {
            debug!("DeepTrap garants immunity against DoT");
            return;
        }
        // apply HP burning
        let n = self.effects[actor].get(Effect::HPBurning);
        if n > 0 {
            if let Some(inflictor) = self.effects[actor].get_last_inflictor(Effect::HPBurning) {
                let mut hp_burn_dmg: f32 = self.get_max_health(actor) * 0.08 * n as f32;
                let mastery = self.get_mastery(inflictor);
                hp_burn_dmg *= 1.0 + mastery;
                let max = 0.3 * self.get_max_health(inflictor);
                if hp_burn_dmg > max {
                    debug!("{} HP burning damage capped at {}", self.name(actor), max);
                    hp_burn_dmg = max;
                }
                self.damage_hp_burning(inflictor, actor, hp_burn_dmg);
            } else {
                warn!("No inflictor for HP burning on {}", self.name(actor));
            }
        }
    }
}
