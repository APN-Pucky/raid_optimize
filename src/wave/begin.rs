use crate::{indent, debug, data::{passive::Passive, skill::{SkillData, Skill}}};

use super::Wave;

impl<const LEN:usize> Wave<'_,LEN> {
    pub fn begin(&mut self) {
        debug!("Wave begin");
        indent!({
            self.on_begin_wave_space();
            self.on_begin_wave_marville();
        })
    }
}