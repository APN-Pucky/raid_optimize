use derive_macro::Cooldown;

use crate::wave::heroes::{Cooldown, Skilled, Typed, Selector, Execute};
use crate::{wave::{Wave, InstanceIndex, heroes::{PassiveSkill }}, data::{skill::{Skill, SkillType, Select}, effect::{Effect}, }, };

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct Resplendence{
    pub turn_meter_ratio: f32,
}

impl Default for Resplendence{
    fn default() -> Self {
        Self {
            turn_meter_ratio: 0.,
        }
    }
}

impl PassiveSkill for Resplendence {}

impl Cooldown for Resplendence {
    fn get_cooldown(&self) -> u32 {
        0
    }
}