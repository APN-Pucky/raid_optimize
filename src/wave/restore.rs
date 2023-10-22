use crate::{
    data::{skill::Skill, subskill::Trigger},
    debug, indent, roll, warn,
    wave::{for_skill, heroes::agatha::aristocratic_style::AristocraticStyle, stat::Stat},
};

use super::{InstanceIndex, Wave};

impl Wave<'_> {
    pub fn heal(&mut self, actor: InstanceIndex, target: InstanceIndex, health: f32) {
        if self.is_dead(target) {
            warn!(
                "{} is dead, cannot heal [{},{}]",
                self.name(target),
                self.health[target],
                self.health[target] > 0.0
            );
            return;
        }
        for i in self.get_enemies_indices(target) {
            for_skill!(
                self,
                i,
                Skill::AristocraticStyle(AristocraticStyle {
                    steal_shield_and_heal_chance
                }),
                {
                    if roll(steal_shield_and_heal_chance) {
                        debug!("{} transfers heal from {}", self.name(i), self.name(target));
                        self.heal(i, i, health);
                        return;
                    }
                }
            );
        }

        let healing_effect = self.get_healing_effect(actor);
        let healed_effect = self.get_healed_effect(target);
        let heal = health * (healing_effect + healed_effect); // TODO handle rounding
        let new_health = self.get_max_health(target).min(self.health[target] + heal);
        debug!(
            "{} heals {} health (healing_effect: {}, healed_effect: {})",
            self.name(target),
            heal,
            healing_effect,
            healed_effect
        );
        self.add_stat(target, Stat::HealthHealed, new_health - self.health[target]);
        self.health[target] = new_health;
        self.on_trigger(target, Trigger::Healed);
        self.on_trigger(actor, Trigger::Healing);
    }

    pub fn restore(&mut self, actor: InstanceIndex, target: InstanceIndex, health: f32) {
        //if self.is_dead(target) {
        //    return;
        //}
        debug!(
            "{} restores {} for {}",
            self.name(actor),
            self.name(target),
            health
        );
        indent!({
            self.add_stat(actor, Stat::HealthRestored, health);
            self.heal(actor, target, health)
        });
    }

    pub fn restore_single(&mut self, actor: InstanceIndex, target: InstanceIndex, health: f32) {
        self.restore(actor, target, health);
    }

    pub fn restore_ally_team(&mut self, actor: InstanceIndex, restore_hp: f32) {
        debug!("{} restores own team for {}", self.name(actor), restore_hp);
        indent!({
            for i in self.get_ally_indices(actor) {
                self.restore_single(actor, i, restore_hp);
            }
        })
    }

    pub fn restore_max_hp_ratio_own_team(
        &mut self,
        actor: InstanceIndex,
        restore_max_hp_ratio: f32,
    ) {
        debug!(
            "{} restores own team by {} of their max_hp ",
            self.name(actor),
            restore_max_hp_ratio
        );
        indent!({
            for i in self.get_ally_indices(actor) {
                self.restore_single(actor, i, self.get_max_health(i) * restore_max_hp_ratio);
            }
        })
    }

    pub fn restore_to_highest_ally_health_percentage(&mut self, actor: InstanceIndex) {
        debug!(
            "{} restores own team to highest ally health percentage",
            self.name(actor)
        );
        indent!({
            let a = self.get_highest_health_percentage_ally(actor);
            let hp = self.health[a] / self.get_max_health(a);
            for a in self.get_ally_indices(actor) {
                self.heal(actor, a, self.get_max_health(a) * hp - self.health[a]);
            }
        })
    }
}
