use crate::{data::skill::Skill, debug, wave::Wave};

use self::resplendence::Resplendence;

pub mod fission_of_life;
pub mod nightmare;
pub mod resplendence;
pub mod tricks;

impl Wave<'_> {
    pub fn on_begin_wave_space(&mut self) {
        self.get_indices_iter().for_each(|i| {
            if let [Skill::Resplendence(Resplendence {
                turn_meter_ratio, ..
            }), ..] = self.heroes[i].skills[..]
            {
                debug!("{} has Resplendence", self.fmt(i));
                self.set_turn_meter(i, self.turn_meter_threshold * turn_meter_ratio);
            }
        });
    }
}
#[cfg(test)]
mod tests;
