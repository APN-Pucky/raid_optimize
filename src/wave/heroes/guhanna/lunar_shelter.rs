use derive_macro::PassiveSkill;

use crate::wave::heroes::Cooldown;
use crate::wave::heroes::PassiveSkill;

#[derive(PassiveSkill, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct LunarShelter {
    pub direct_dmg_reduction: f32,
}

impl Default for LunarShelter {
    fn default() -> Self {
        Self {
            direct_dmg_reduction: 0.12,
        }
    }
}
