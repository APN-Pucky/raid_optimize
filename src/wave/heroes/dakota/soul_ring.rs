use crate::wave::heroes::{Cooldown, Skilled, Typed, Selector, Execute};
use crate::{wave::{Wave, InstanceIndex, heroes::{PassiveSkill }}, data::{skill::{Skill, SkillType, Select}, effect::{Effect}, }, };

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct SoulRing {
    pub effect_res_down_chance : f32,
    pub effect_res_down_turns : u32,
}

impl Default for SoulRing{
    fn default() -> Self {
        Self {
            effect_res_down_chance : 1.,
            effect_res_down_turns : 2,
        }
    }
}

impl PassiveSkill for SoulRing{}

impl Cooldown for SoulRing {
    fn get_cooldown(&self) -> u32 {
        0
    }
}