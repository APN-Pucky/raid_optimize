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
pub struct HolyCreature {
    pub increase_self_turn_meter_ratio: f32,
    pub increase_ally_turn_meter_ratio: f32,
    pub increase_tenacity_ratio: f32,
    pub increase_tenacity_ratio_max: f32,
}

impl Default for HolyCreature {
    fn default() -> Self {
        Self {
            increase_self_turn_meter_ratio: 0.10,
            increase_ally_turn_meter_ratio: 0.10,
            increase_tenacity_ratio: 0.5,
            increase_tenacity_ratio_max: 0.63,
        }
    }
}
