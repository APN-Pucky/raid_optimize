use crate::wave::heroes::Cooldown;
use crate::wave::heroes::PassiveSkill;

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct AristocraticStyle {
    pub steal_shield_and_heal_chance: f32,
}

impl Default for AristocraticStyle {
    fn default() -> Self {
        Self {
            steal_shield_and_heal_chance: 0.3,
        }
    }
}

impl PassiveSkill for AristocraticStyle {}

impl Cooldown for AristocraticStyle {
    fn get_cooldown(&self) -> u32 {
        0
    }
}
