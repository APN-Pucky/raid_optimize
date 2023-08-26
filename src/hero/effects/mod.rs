use enum_map::{EnumMap, Enum, EnumArray};
use serde::Deserialize;

use crate::wave::InstanceRef;



pub mod buff;
pub mod debuff;
pub mod unique;
pub mod map;

use self::buff::Buff;
use self::buff::attribute::AttributeBuff;

use self::debuff::Debuff;
use self::debuff::attribute::AttributeDebuff;
use self::debuff::dot::DotDebuff;

use self::map::EffectsMap;
use self::unique::Unique;
#[derive(Debug)]
pub struct Effects {
    // blue, positive, expire usually
    pub buffs: EffectsMap<Buff>, 
    pub attribute_buffs: EffectsMap<AttributeBuff>, 
    // red, negative, expire usually
    pub debuffs: EffectsMap<Debuff>, 
    pub attribute_debuffs: EffectsMap<AttributeDebuff>,
    pub dot_debuffs: EffectsMap<DotDebuff>, 
    // purple, don't expire usually
    pub unique: EffectsMap<Unique>, 
}

impl Default for Effects {
    fn default() -> Self {
        Self::new()
    }
}
impl Effects {
    pub fn new() -> Effects {
        Effects {
            buffs: EffectsMap::new(),
            attribute_buffs: EffectsMap::new(),
            debuffs: EffectsMap::new(),
            attribute_debuffs: EffectsMap::new(),
            dot_debuffs: EffectsMap::new(),
            unique: EffectsMap::new(),
        }
    }

    pub fn reduce(&mut self) {
        self.buffs.reduce();
        self.attribute_buffs.reduce();
        self.debuffs.reduce();
        self.attribute_debuffs.reduce();
        self.dot_debuffs.reduce();
        //self.unique.reduce();
    }

    pub fn remove_empty(&mut self) {
        self.buffs.remove_empty();
        self.attribute_buffs.remove_empty();
        self.debuffs.remove_empty();
        self.attribute_debuffs.remove_empty();
        self.dot_debuffs.remove_empty();
        self.unique.remove_empty();
    }
}