use derive_macro::PassiveSkill;

use crate::wave::heroes::Cooldown;
use crate::wave::heroes::PassiveSkill;

#[derive(PassiveSkill, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct BloodlustStrike {
    pub leech: f32,
    pub damage_reduction_buffs: f32,
    pub damage_reduction_nobuffs: f32,
}
impl Default for BloodlustStrike {
    fn default() -> Self {
        Self {
            leech: 0.16,
            damage_reduction_buffs: 0.18,
            damage_reduction_nobuffs: 0.25,
        }
    }
}
