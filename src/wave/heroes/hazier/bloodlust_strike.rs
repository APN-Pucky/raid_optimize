

use crate::wave::heroes::{Cooldown};
use crate::{wave::{heroes::{PassiveSkill }}, };

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct BloodlustStrike {
    pub leech : f32,
    pub damage_reduction_buffs : f32,
    pub damage_reduction_nobuffs : f32,
}
impl Default for BloodlustStrike{
    fn default() -> Self {
        Self {
            leech : 0.16,
            damage_reduction_buffs : 0.18,
            damage_reduction_nobuffs : 0.25,
        }
    }
}

impl PassiveSkill for BloodlustStrike{}

impl Cooldown for BloodlustStrike {
    fn get_cooldown(&self) -> u32 {
        0
    }
}