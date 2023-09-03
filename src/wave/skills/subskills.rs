use crate::{data::{skill::{Skill, get_cooldown, is_basic_attack}, faction::Faction, effect::Effect, subskill::{Scale, SubSkill, Target, Type}}, indent, debug};

use super::{InstanceIndex, Wave};



impl<const LEN:usize> Wave<'_,LEN> {
    pub fn execute_subskill(&mut self,subskill : &SubSkill, actor :InstanceIndex, target :InstanceIndex,  skill: &Skill) {
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
                val= wave.get_max_health(target) * subskill.ratio;
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
                targets  = vec![target];
            },
            Target::SingleEnemy => {
                targets  = vec![target];
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