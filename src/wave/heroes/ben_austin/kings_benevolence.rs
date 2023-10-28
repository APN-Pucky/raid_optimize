use derive_macro::PassiveSkill;

use crate::data::skill::Skill;
use crate::wave::for_skill;
use crate::wave::has_skill;
use crate::wave::heroes::Cooldown;
use crate::wave::heroes::PassiveSkill;
use crate::wave::InstanceIndex;
use crate::wave::Wave;
use ordered_float::OrderedFloat;

#[derive(PassiveSkill, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct KingsBenevolence {
    pub cleanse_chance: f32,
    pub effect_resistance_turns: u32,
}

impl Default for KingsBenevolence {
    fn default() -> Self {
        Self {
            cleanse_chance: 0.28,
            effect_resistance_turns: 2,
        }
    }
}
