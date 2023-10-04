use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill, SkillData}, effect::Effect, }, debug, };

use self::resplendence::Resplendence;

pub mod fission_of_life;
pub mod tricks;
pub mod nightmare;
pub mod resplendence;


impl Wave<'_> {
    pub fn on_begin_wave_space(&mut self) {
        (0..self.len())
                .for_each(|i| 
                    match self.heroes[i].skills[..] {
                        [ Skill { data : SkillData::Resplendence(Resplendence{turn_meter_ratio}), ..} ,.. ] => {
                            debug!("{} has Resplendence", self.fmt(i));
                            self.set_turn_meter(i,self.turn_meter_threshold * turn_meter_ratio);
                        },
                        _ => {}
                    }
                );
    }
}
#[cfg(test)]
mod tests;