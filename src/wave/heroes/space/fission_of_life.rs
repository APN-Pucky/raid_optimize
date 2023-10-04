use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill}, effect::{Effect}, }, };

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct FissionOfLife{
    pub restore_max_hp_ratio : f32,
    pub heal_turns : u32,
    pub increase_turn_meter_ratio : f32,
}

impl Default for FissionOfLife{
    fn default() -> Self {
        Self {
            restore_max_hp_ratio : 0.1,
            heal_turns : 2,
            increase_turn_meter_ratio : 0.2,
        }
    }
}

impl FissionOfLife{
    pub fn execute(&self, wave : &mut Wave,  skill : &Skill, actor:InstanceIndex, defender:InstanceIndex, ) {
       wave.restore_max_hp_ratio_own_team(actor, self.restore_max_hp_ratio);
       wave.inflict_ally_team(actor, Effect::Heal,1.0, self.heal_turns);
       wave.increase_turn_meter_team(actor, self.increase_turn_meter_ratio);
    }
}
