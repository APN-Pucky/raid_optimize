use derive_macro::PassiveSkill;

use crate::wave::heroes::Cooldown;
use crate::wave::heroes::PassiveSkill;

#[derive(PassiveSkill, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct CounterattackCommand {
    pub blades: u32,
    pub crit_damage_turns: u32,
    pub attack_damage_ratio: f32,
}

impl Default for CounterattackCommand {
    fn default() -> Self {
        Self {
            blades: 4,
            crit_damage_turns: 1,
            attack_damage_ratio: 2.3,
        }
    }
}
