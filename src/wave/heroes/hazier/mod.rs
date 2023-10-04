use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill, SkillData}, effect::{Effect, is_buff}}, indent, debug};

pub mod darknight_arbitrament;
pub mod darknight_strike;
pub mod eye_for_an_eye;

#[cfg(test)]
mod tests;