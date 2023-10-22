use crate::data::effect::Effect;

use super::{InstanceIndex, Wave};
use enum_map::EnumMap;

#[derive(Debug)]
pub struct Effects {
    pub em: EnumMap<Effect, Vec<(u32, u32, InstanceIndex)>>, // effect , for layer : [cur turns , start turns,  inflictor] // TODO make it an enum
                                                             //pub vm : [Vec<(u32,InstanceRef)>;Effect::NumberOfEffects as usize],
}

impl Default for Effects {
    fn default() -> Self {
        Self::new()
    }
}

impl Effects {
    pub fn new() -> Effects {
        Effects {
            em: EnumMap::default(),
        }
    }

    pub fn remove_layers(&mut self, key: Effect, n: u32) {
        for _ in 0..n {
            self.remove_layer(key);
        }
    }

    pub fn remove_layer(&mut self, key: Effect) {
        self.em[key].pop();
    }

    pub fn get_last_inflictor(&self, key: Effect) -> Option<InstanceIndex> {
        self.em[key].last().map(|tp| tp.2)
        //if let Some(tp) = self.em[key].last() {
        //    Some(tp.2)
        //}
        //else {
        //    None
        //}
    }

    pub fn clear(&mut self) {
        for (_key, value) in self.em.iter_mut() {
            value.clear();
        }
    }

    pub fn clear_single(&mut self, key: Effect) {
        self.em[key].clear();
    }

    pub fn has(&self, key: Effect) -> bool {
        !self.em[key].is_empty()
    }

    pub fn get(&self, key: Effect) -> u32 {
        self.em[key].len() as u32
    }

    pub fn push(&mut self, key: Effect, turns: u32, ir: InstanceIndex) {
        self.em[key].push((turns, turns, ir));
    }

    pub fn remove_empty(&mut self) {
        // remove zero elements from effect vectors
        for (_key, value) in self.em.iter_mut() {
            value.retain(|&(x, _, _)| x > 0);
        }
    }

    pub fn reduce(&mut self) {
        for (_key, value) in self.em.iter_mut() {
            let mut i = 0;
            while i < value.len() {
                value[i].0 -= 1;
                if value[i].0 == 0 {
                    value.remove(i);
                } else {
                    i += 1;
                }
            }
        }
        //self.remove_empty();
    }
}

impl Wave<'_> {
    pub fn has_effect(&self, actor: InstanceIndex, effect: Effect) -> bool {
        self.effects[actor].has(effect)
    }

    pub fn has_debuff(&self, actor: InstanceIndex) -> bool {
        for (key, value) in self.effects[actor].em.iter() {
            if key.is_debuff() && !value.is_empty() {
                return true;
            }
        }
        false
    }

    pub fn has_buff(&self, actor: InstanceIndex) -> bool {
        for (key, value) in self.effects[actor].em.iter() {
            if key.is_buff() && !value.is_empty() {
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
        for (_key, value) in self.effects[actor].em.iter() {
            // check if actor in value
            for (_turns, _, ir) in value.iter() {
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

    pub fn get_number_of_buff_layers(&self, actor: InstanceIndex) -> u32 {
        let mut n = 0;
        for (k, v) in self.effects[actor].em.iter() {
            if k.is_buff() {
                n += v.len() as u32;
            }
        }
        n
    }
}
