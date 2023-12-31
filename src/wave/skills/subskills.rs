use crate::{
    data::{
        effect::Effect,
        skill::{Generic, Skill},
        subskill::{Scale, SubSkill, Target, Trigger, Triggerer, Type},
    },
    debug,
};

use super::{InstanceIndex, Wave};

impl Wave<'_> {
    pub fn on_trigger(
        &mut self,
        triggerer: InstanceIndex,
        trigger: Trigger, //        ,trigger_effect : Option<Effect>
    ) {
        for actor in self.get_indices_iter() {
            for s in &self.heroes[actor].skills {
                if let Skill::Generic(Generic { subskills, .. }) = s {
                    for ss in subskills {
                        if ss.trigger == trigger {
                            //if let Some(te) = ss.trigger_effect {
                            //    if ss.trigger_effect != trigger_effect {
                            //        continue;
                            //    }
                            //}
                            if (ss.triggerer == Triggerer::Any)
                                || (ss.triggerer == Triggerer::Ally
                                    && self.are_allies(actor, triggerer))
                                || (ss.triggerer == Triggerer::Enemy
                                    && self.are_enemies(actor, triggerer))
                                || (ss.triggerer == Triggerer::I && actor == triggerer)
                            {
                                self.execute_subskill(&ss, actor, Some(triggerer), s);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn execute_subskill(
        &mut self,
        subskill: &SubSkill,
        actor: InstanceIndex,
        target: Option<InstanceIndex>,
        skill: &Skill,
    ) {
        let mut val = 0.0;
        let effect = subskill.effect;
        let chance = subskill.chance;
        let turns = subskill.turns;
        if !self.is_alive(actor) {
            debug!(
                "{} is dead -> can't execute subskill",
                self.heroes[actor].name
            );
            return;
        }
        if self.has_effect(actor, Effect::Imprison) {
            debug!(
                "{} is imprisoned -> can't execute subskill",
                self.heroes[actor].name
            );
            return;
        }
        match subskill.scale {
            Scale::Attack => {
                val = self.get_attack_damage(actor) * subskill.ratio;
            }
            Scale::Defense => {
                val = self.get_defense(actor) * subskill.ratio;
            }
            Scale::MaxHealth => {
                val = self.get_max_health(actor) * subskill.ratio;
            }
            Scale::TargetMaxHealth => {
                val = self.get_max_health(target.expect("TargetMaxHealth needs a target"))
                    * subskill.ratio;
            }
            Scale::None => {}
        }
        let targets: Vec<InstanceIndex> = match subskill.target {
            Target::Everyone => self.get_indices_iter().collect(),
            Target::SingleAlly => {
                vec![target.expect("SingleAlly needs a target")]
            }
            Target::SingleEnemy => {
                vec![target.expect("SingleEnemy needs a target")]
            }
            Target::AllEnemies => self.get_enemies_indices(actor),
            Target::AllAllies => self.get_ally_indices(actor),
            Target::SingleSelf => {
                vec![actor]
            }
            Target::None => {
                vec![]
            }
            Target::LowestHealthAlly => {
                vec![self.get_lowest_health_ally(actor)]
            }
            Target::LowestHealthPercentageAlly => {
                vec![self.get_lowest_health_percentage_ally(actor)]
            }
        };

        match subskill.typ {
            Type::ActAgain => {
                self.act(actor);
            }
            Type::Damage => {
                for target in targets.iter() {
                    self.attack_single(actor, *target, val, skill);
                }
            }
            Type::Shield => {
                for target in targets.iter() {
                    self.shield_single(actor, *target, val, turns);
                }
            }
            Type::Restore => {
                for target in targets.iter() {
                    self.restore_single(actor, *target, val);
                }
            }
            Type::Inflict => {
                // Inflict is always implicit!
            }
            Type::RemoveAllBuffs => {
                for target in targets.iter() {
                    self.remove_all_buffs_single(actor, *target);
                }
            }
            Type::ReduceTurnMeter => {
                for target in targets.iter() {
                    self.reduce_turn_meter_ratio(actor, *target, subskill.ratio);
                }
            }
            Type::IncreaseTurnMeter => {
                for target in targets.iter() {
                    self.increase_turn_meter_ratio(actor, *target, subskill.ratio)
                }
            }
            Type::StealTurnMeter => {
                for target in targets.iter() {
                    self.steal_turn_meter_ratio(actor, *target, subskill.ratio)
                }
            }
            Type::RestoreMaxHealth => {
                for target in targets.iter() {
                    self.restore_single(
                        actor,
                        *target,
                        subskill.ratio * self.get_max_health(*target),
                    );
                }
            }
            Type::RemoveEffect => {
                for target in targets.iter() {
                    self.remove_effect_single(actor, *target, effect);
                }
            }
            Type::RemoveAllAttributeDebuffs => {
                for target in targets.iter() {
                    self.remove_effect_filter_single(actor, *target, Effect::is_attribute_debuff);
                }
            }
            Type::ChangeSilence => {
                for target in targets.iter() {
                    if self.effects[*target].get(Effect::Silence) > 0
                        && turns > 0
                        && effect != Effect::None
                    {
                        self.remove_effect_single(actor, *target, Effect::Silence);
                        self.inflict_single(actor, *target, effect, 1.0, turns);
                    }
                }
            }
        }
        if chance > 0.0 && turns > 0 && effect != Effect::None {
            for target in targets.iter() {
                self.inflict_single(actor, *target, effect, chance, turns);
            }
        }
    }
}
