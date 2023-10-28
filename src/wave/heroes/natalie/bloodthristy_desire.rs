use derive_macro::PassiveSkill;

// TODO needs implementation
use crate::wave::heroes::Cooldown;
use crate::wave::heroes::PassiveSkill;

#[derive(PassiveSkill, Debug, Default, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct BloodthirstyDesire {}
