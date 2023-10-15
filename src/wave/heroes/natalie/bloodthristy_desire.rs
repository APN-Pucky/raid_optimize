// TODO needs implementation
use crate::wave::heroes::Cooldown;
use crate::wave::heroes::PassiveSkill;

#[derive(Debug, Default, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct BloodthirstyDesire {}

impl PassiveSkill for BloodthirstyDesire {}

impl Cooldown for BloodthirstyDesire {
    fn get_cooldown(&self) -> u32 {
        0
    }
}
