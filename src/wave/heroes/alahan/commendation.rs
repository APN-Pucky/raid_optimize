use crate::{wave::{Wave, InstanceIndex, heroes::{PassiveSkill, Cooldown}, }, data::{skill::{Skill, SkillType, Select, get_cooldown}, effect::{Effect}, }, };

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct Commendation {
    pub cooldown : u32,
    pub max_hp_restore_ratio: f32,
    pub attack_up_turns : u32,
}

impl Default for Commendation {
    fn default() -> Self {
        Self {
            cooldown :1 ,
            max_hp_restore_ratio: 0.1,
            attack_up_turns : 2,
        }
    }
}


impl PassiveSkill for Commendation {}
//impl Cooldown for Commendation {
//    fn get_cooldown(&self) -> u32 {
//        self.cooldown
//    }
//}