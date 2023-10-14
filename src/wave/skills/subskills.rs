use crate::{data::{skill::{Skill, Generic}, effect::Effect, subskill::{Scale, SubSkill, Target, Type, Trigger}}};

use super::{InstanceIndex, Wave};





impl Wave<'_> {

    pub fn on_trigger(&mut self, actor : InstanceIndex, trigger : Trigger) {
        for s in &self.heroes[actor].skills {
            match s {
                Skill::Generic(Generic{subskills , ..}) => {
                    for ss in subskills {
                        if ss.trigger ==  trigger {
                            self.execute_subskill(&ss, actor, None,s);
                        }
                    }
                }
                _ => { }
            }
        }
    }

    pub fn execute_subskill(&mut self,subskill : &SubSkill, actor :InstanceIndex, target :Option<InstanceIndex>,  skill: &Skill) {
        let wave = self;
        let mut val= 0.0;
        let mut targets : Vec<InstanceIndex> = vec![];
        let mut effect = Effect::None;
        let mut chance = 0.0;
        let mut turns = 0;
        match subskill.scale {
            Scale::AttackDamage => {
                val= wave.get_attack_damage(actor) * subskill.ratio;
            },
            Scale::MaxHealth => {
                val= wave.get_max_health(actor) * subskill.ratio;
            },
            Scale::TargetMaxHealth => {
                val= wave.get_max_health(target.expect("TargetMaxHealth needs a target")) * subskill.ratio;
            },
            Scale::None => {},
        }
        match subskill.effect {
            Effect::None => {},
            _ => {
                effect = subskill.effect;
                chance = subskill.chance;
                turns = subskill.turns;
            },
        }
        match subskill.target {
            Target::Everyone => {
                // 0..LEN
                targets = wave.get_indices();
            },
            Target::SingleAlly => {
                targets  = vec![target.expect("SingleAlly needs a target")];
            },
            Target::SingleEnemy => {
                targets  = vec![target.expect("SingleEnemy needs a target")];
            },
            Target::AllEnemies => {
                targets = wave.get_enemies_indices(actor);
            },
            Target::AllAllies => {
                targets = wave.get_ally_indices(actor);
            },
            Target::SingleSelf => {
                targets = vec![actor];
            },
            Target:: None => {
                targets = vec![];
            },
            Target::LowestHealthAlly => {
                targets = vec![wave.get_lowest_health_ally(actor)];
            }
        } 
        match subskill.typ {
            Type::Damage => {
                for target in targets {
                    wave.attack_single(actor,target,val,skill);
                }
            },
            Type::Shield => {
                for target in targets {
                    wave.shield_single(actor,target,val,turns);
                }
            }
            Type::Restore => {
                for target in targets {
                    wave.restore_single(actor,target,val);
                }
            },
            Type::Inflict => {
                for target in targets {
                    wave.inflict_single(actor,target,effect,chance,turns);
                }
            },
            Type::RemoveAllBuffs => {
                for target in targets {
                    wave.remove_all_buffs_single(actor,target);
                }
            }
        }
    }
}