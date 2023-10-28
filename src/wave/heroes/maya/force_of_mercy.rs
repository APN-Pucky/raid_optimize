use derive_macro::PassiveSkill;

use crate::wave::heroes::Cooldown;
use crate::wave::heroes::PassiveSkill;

#[derive(PassiveSkill, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct ForceOfMercy {
    pub max_hp_restore_ratio: f32,
    pub healing_effect: f32,
}

impl Default for ForceOfMercy {
    fn default() -> Self {
        Self {
            max_hp_restore_ratio: 0.026,
            healing_effect: 0.3,
        }
    }
}
