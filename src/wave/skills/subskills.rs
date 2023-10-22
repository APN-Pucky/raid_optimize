use crate::data::{
    effect::Effect,
    skill::{Generic, Skill},
    subskill::{Scale, SubSkill, Target, Trigger, Triggerer, Type},
};

use super::{InstanceIndex, Wave};

impl Wave<'_> {
    pub fn on_trigger(&mut self, triggerer: InstanceIndex, trigger: Trigger) {
        for actor in self.get_indices_iter() {
            for s in &self.heroes[actor].skills {
                if let Skill::Generic(Generic { subskills, .. }) = s {
                    for ss in subskills {
                        if ss.trigger == trigger {
                            if (ss.triggerer == Triggerer::Any)
                                || (ss.triggerer == Triggerer::Ally
                                    && self.are_allies(actor, triggerer))
                                || (ss.triggerer == Triggerer::Enemy
                                    && self.are_enemies(actor, triggerer))
                                || (ss.triggerer == Triggerer::I && actor == triggerer)
                            {
                                self.execute_subskill(&ss, actor, None, s);
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
        let wave = self;
        let mut val = 0.0;
        let mut effect = Effect::None;
        let mut chance = 0.0;
        let mut turns = 0;
        match subskill.scale {
            Scale::Attack => {
                val = wave.get_attack_damage(actor) * subskill.ratio;
            }
            Scale::Defense => {
                val = wave.get_defense(actor) * subskill.ratio;
            }
            Scale::MaxHealth => {
                val = wave.get_max_health(actor) * subskill.ratio;
            }
            Scale::TargetMaxHealth => {
                val = wave.get_max_health(target.expect("TargetMaxHealth needs a target"))
                    * subskill.ratio;
            }
            Scale::None => {}
        }
        match subskill.effect {
            Effect::None => {}
            _ => {
                effect = subskill.effect;
                chance = subskill.chance;
                turns = subskill.turns;
            }
        }
        let targets: Vec<InstanceIndex> = match subskill.target {
            Target::Everyone => wave.get_indices_iter().collect(),
            Target::SingleAlly => {
                vec![target.expect("SingleAlly needs a target")]
            }
            Target::SingleEnemy => {
                vec![target.expect("SingleEnemy needs a target")]
            }
            Target::AllEnemies => wave.get_enemies_indices(actor),
            Target::AllAllies => wave.get_ally_indices(actor),
            Target::SingleSelf => {
                vec![actor]
            }
            Target::None => {
                vec![]
            }
            Target::LowestHealthAlly => {
                vec![wave.get_lowest_health_ally(actor)]
            }
        };
        match subskill.typ {
            Type::Damage => {
                for target in targets.iter() {
                    wave.attack_single(actor, *target, val, skill);
                }
            }
            Type::Shield => {
                for target in targets.iter() {
                    wave.shield_single(actor, *target, val, turns);
                }
            }
            Type::Restore => {
                for target in targets.iter() {
                    wave.restore_single(actor, *target, val);
                }
            }
            Type::Inflict => {
                // Inflict is always implicit!
            }
            Type::RemoveAllBuffs => {
                for target in targets.iter() {
                    wave.remove_all_buffs_single(actor, *target);
                }
            }
            Type::ReduceTurnMeter => {
                for target in targets.iter() {
                    wave.reduce_turn_meter_ratio(actor, *target, subskill.ratio);
                }
            }
            Type::IncreaseTurnMeter => {
                for target in targets.iter() {
                    wave.increase_turn_meter_ratio(actor, *target, subskill.ratio)
                }
            }
        }
        if chance > 0.0 && turns > 0 && effect != Effect::None {
            for target in targets.iter() {
                wave.inflict_single(actor, *target, effect, chance, turns);
            }
        }
    }
}
