use derive_macro::PassiveSkill;

use crate::wave::heroes::Cooldown;
use crate::wave::heroes::PassiveSkill;

#[derive(PassiveSkill, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct SoulRing {
    pub effect_res_down_chance: f32,
    pub effect_res_down_turns: u32,
}

impl Default for SoulRing {
    fn default() -> Self {
        Self {
            effect_res_down_chance: 1.,
            effect_res_down_turns: 2,
        }
    }
}
