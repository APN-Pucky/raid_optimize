use crate::{indent, debug, data::{passive::Passive, skill::{SkillData, Skill}}};

use super::Wave;

impl<const LEN:usize> Wave<'_,LEN> {
    pub fn begin(&mut self) {
        debug!("Wave begin");
        indent!({
            (0..LEN)
                .for_each(|i| 
                    match self.heroes[i].skills[..] {
                        [ Skill { data : SkillData::Resplendence { turn_meter_ratio }, ..} ,.. ] => {
                            debug!("{} has Resplendence", self.fmt(i));
                            self.set_turn_meter(i,self.turn_meter_threshold * turn_meter_ratio);
                        },
                        _ => {}
                    }
                );
        })
    }
}