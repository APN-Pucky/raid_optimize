use enum_map::EnumMap;

use crate::{wave::InstanceIndex};

use super::effect::Effect;

#[derive(Debug)]
pub struct Effects {
    pub em : EnumMap<Effect,Vec<(u32,InstanceIndex)>>,
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
            em : EnumMap::default(),
        }
    }

    pub fn clear(&mut self) {
        for (_key,value) in self.em.iter_mut() {
            value.clear();
        }
    }

    pub fn has(&self, key: Effect) -> bool {
        !self.em[key].is_empty()
    }

    pub fn get(&self, key: Effect) -> u32 {
        self.em[key].len() as u32
    }

    pub fn push(&mut self, key: Effect, turns : u32, ir:InstanceIndex) {
        self.em[key ].push((turns,ir));
    }

    pub fn remove_empty(&mut self) {
        // remove zero elements from effect vectors
        for (_key,value) in self.em.iter_mut() {
            value.retain(|&(x,_)| x > 0);
        }
    }

    pub fn reduce(&mut self) {
        for (_key,value) in self.em.iter_mut() {
            let mut i = 0;
            while i < value.len() {
                value[i].0 -= 1;
                if value[i].0 == 0 {
                    value.remove(i);
                }
                else {
                    i += 1;
                }
            } 
        }
        //self.remove_empty();
    }
}