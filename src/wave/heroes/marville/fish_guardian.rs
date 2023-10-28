use derive_macro::PassiveSkill;

use crate::wave::heroes::Cooldown;
use crate::wave::heroes::PassiveSkill;

#[derive(PassiveSkill, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct FishGuardian {
    pub restore_fish_shoal: u32,
    pub max_hp_restore_ratio: f32,
    pub damage_reduction: f32,
}

impl Default for FishGuardian {
    fn default() -> Self {
        Self {
            restore_fish_shoal: 3,
            max_hp_restore_ratio: 0.48,
            damage_reduction: 0.1,
        }
    }
}
