use crate::wave::heroes::Cooldown;
use crate::wave::heroes::PassiveSkill;

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
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

impl PassiveSkill for LunarShelter {}

impl Cooldown for LunarShelter {
    fn get_cooldown(&self) -> u32 {
        0
    }
}
