use crate::{data::subskill::Trigger, debug, indent};

use super::Wave;

impl Wave<'_> {
    pub fn begin(&mut self) {
        debug!("Wave begin");
        indent!({
            for i in self.get_indices_iter() {
                self.on_trigger(i, Trigger::WaveBegin);
            }
            // TODO these can be converted to subskills
            self.on_begin_wave_space();
            self.on_begin_wave_marville();
        })
    }
}
