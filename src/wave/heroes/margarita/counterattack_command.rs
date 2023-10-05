

use crate::wave::heroes::{Cooldown, Skilled, Typed, Selector, Execute};
use crate::{wave::{Wave, InstanceIndex, heroes::{PassiveSkill }}, data::{skill::{Skill, SkillType, Select}, effect::{Effect}, }, };

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct CounterattackCommand {
    pub blades : u32,
    pub crit_damage_turns : u32,
    pub attack_damage_ratio : f32,
}

impl Default for CounterattackCommand{
    fn default() -> Self {
        Self {
            blades : 4,
            crit_damage_turns : 1,
            attack_damage_ratio : 2.3,
        }
    }
}

impl PassiveSkill for CounterattackCommand{}

impl Cooldown for CounterattackCommand{
    fn get_cooldown(&self) -> u32 {
        0
    }
}