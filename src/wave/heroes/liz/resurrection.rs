use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill}, effect::{Effect, is_dot}, }, };

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct Resurrection {
    pub        shield_max_hp_ratio: f32,
    pub    shield_turns: u32,
    pub    cleanse_dot_debuffs: u32,
    pub    restore_max_hp_ratio: f32,
}

impl Default for Resurrection {
    fn default() -> Self {
        Self {
            shield_max_hp_ratio: 0.2,
            shield_turns: 2,
            cleanse_dot_debuffs: 3,
            restore_max_hp_ratio: 0.1,
        }
    }
}

impl Resurrection {
    pub fn execute(&self, wave : &mut Wave,  skill : &Skill, actor:InstanceIndex, target:InstanceIndex, ) {
        let max_hp = wave.get_max_health(actor);
        wave.restore_max_hp_ratio_own_team(actor,self.restore_max_hp_ratio);
        wave.shield_ally_team(actor,self.shield_max_hp_ratio * max_hp  ,self.shield_turns);
        wave.cleanse_team(actor,&is_dot,self.cleanse_dot_debuffs);
    }
}
