
use crate::wave::heroes::{Cooldown, Skilled, Typed, Selector, Execute};
use crate::{wave::{Wave, InstanceIndex, heroes::{PassiveSkill }}, data::{skill::{Skill, SkillType, Select}, effect::{Effect}, }, };

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct FishGuardian {
    pub restore_fish_shoal: u32,
    pub max_hp_restore_ratio : f32,
    pub damage_reduction : f32
}

impl Default for FishGuardian{
    fn default() -> Self {
        Self {
            restore_fish_shoal : 3,
            max_hp_restore_ratio : 0.48,
            damage_reduction : 0.1
        }
    }
}

impl PassiveSkill for FishGuardian{}

impl Cooldown for FishGuardian{
    fn get_cooldown(&self) -> u32 {
        0
    }
}