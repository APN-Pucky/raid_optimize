use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill}, effect::{Effect}, }, };

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct Resplendence{
    turn_meter_ratio: f32,
}

impl Default for Resplendence{
    fn default() -> Self {
        Self {
            attack_damage_ratio : 2.0,
            reduce_speed_chance : 0.6,
            reduce_speed_turns : 2,
            increase_speed_turns : 2,
        }
    }
}

impl Resplendence{
    pub fn execute(&self, wave : &mut Wave,  skill : &Skill, attacker:InstanceIndex, defender:InstanceIndex, ) {
        wave.attack_single(attacker,defender, wave.get_attack_damage(attacker) * self.attack_damage_ratio,skill);
        wave.inflict_single(attacker,defender,Effect::SpeedDownII, self.reduce_speed_chance, self.reduce_speed_turns);
        wave.inflict_ally_team(attacker, Effect::SpeedUpI, 1.0, self.increase_speed_turns);
        //TODO target make no sense here
        //attacker.inflict(defender,Effect::SpeedUpI, 1.0, increase_speed_turns);
    }
}
