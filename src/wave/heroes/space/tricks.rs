use crate::wave::heroes::{Cooldown};
use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill, SkillType, Select}, }, };

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct Tricks{
    pub cooldown : u32,
    pub attack_damage_ratio : f32,
    pub turn_meter_reduction_ratio: f32, 
}

impl Default for Tricks{
    fn default() -> Self {
        Self {
            cooldown : 0,
            attack_damage_ratio : 1.6,
            turn_meter_reduction_ratio: 0.15, 
        }
    }
}

impl Tricks{
    pub const TYPE : SkillType = SkillType::Basic;
    pub const SELECT : Select = Select::AllEnemies;
    pub fn execute(&self, wave : &mut Wave,  skill : &Skill, attacker:InstanceIndex, defender:InstanceIndex, ) {
        wave.attack_single(attacker,defender, wave.get_attack_damage(attacker) * self.attack_damage_ratio, skill);
        wave.reduce_turn_meter(attacker,defender, self.turn_meter_reduction_ratio);
        //TODO target make no sense here
        //attacker.inflict(defender,Effect::SpeedUpI, 1.0, increase_speed_turns);
    }
}
