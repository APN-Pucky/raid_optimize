use enum_map::{EnumMap, Enum, EnumArray};
use serde::Deserialize;

use crate::wave::InstanceRef;

#[derive(Debug)]
pub struct EffectsMap<T> where T: EnumArray<Vec<(u32,InstanceRef)>>
{
    pub em : EnumMap<T,Vec<(u32,InstanceRef)>>,
}

impl<T> Default for EffectsMap<T>  where T: EnumArray<Vec<(u32,InstanceRef)>> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> EffectsMap<T>  where T: EnumArray<Vec<(u32,InstanceRef)>> {
    pub fn new() -> EffectsMap<T> {
        EffectsMap {
            em: EnumMap::default(),
        }
    }

    pub fn has(&self, key: T) -> bool {
        !self.em[key].is_empty()
    }

    pub fn get(&self, key: T) -> u32 {
        self.em[key].len() as u32
    }

    pub fn push(&mut self, key: T, turns : u32, ir:InstanceRef) {
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