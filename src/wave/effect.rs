use crate::{wave::stat::Stat, debug, indent, hero::{instance::Instance, effect::{Effect, is_debuff}}};

use super::{Wave, InstanceIndex};

impl<const LEN:usize> Wave<'_,LEN> {
    pub fn has_effect(&self, actor: InstanceIndex, effect : Effect) -> bool {
        self.effects[actor].has(effect)
    }

    pub fn has_debuff(&self, actor: InstanceIndex) -> bool {
        for (key,value) in self.effects[actor].em.iter() {
            if is_debuff(key) && !value.is_empty() {
                return true;
            }
        }
        false
    }

    pub fn effect_reduce(&mut self, actor: InstanceIndex) {
        self.effects[actor].reduce();
    }

    pub fn count_self_buffs(&self, actor: InstanceIndex) -> usize {
        let mut count = 0;
        for (_key,value) in self.effects[actor].em.iter() {
            // check if actor in value
            for (_turns,ir) in value.iter() {
                if *ir == actor {
                    count += 1;
                    break;
                }
            }
        }
        // shield is also an effect
        for shiedls in self.shields[actor].iter() {
            if shiedls.2 == actor {
                count += 1;
                break;
            }
        }
        count
    }
}