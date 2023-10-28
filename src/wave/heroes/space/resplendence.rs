use derive_macro::PassiveSkill;

use crate::wave::heroes::Cooldown;
use crate::wave::heroes::PassiveSkill;

#[derive(PassiveSkill, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct Resplendence {
    pub turn_meter_ratio: f32,
}

impl Default for Resplendence {
    fn default() -> Self {
        Self {
            turn_meter_ratio: 0.,
        }
    }
}
