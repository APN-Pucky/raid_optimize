use crate::wave::heroes::{Cooldown, Skilled, Typed, Selector, Execute};
use crate::{wave::{Wave, InstanceIndex, heroes::{PassiveSkill }}, data::{skill::{Skill, SkillType, Select}, effect::{Effect}, }, };

#[derive(Debug, Default, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct BloodthirstyDesire {}

impl PassiveSkill for BloodthirstyDesire{}

impl Cooldown for BloodthirstyDesire {
    fn get_cooldown(&self) -> u32 {
        0
    }
}