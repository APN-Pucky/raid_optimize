use crate::{data::subskill::Trigger, debug, indent};

use super::Wave;

impl Wave<'_> {
    pub fn begin(&mut self) {
        debug!("Wave begin");
        indent!({
            for i in self.get_indices_iter() {
                self.on_trigger(i, Trigger::WaveBegin);
            }
            self.on_begin_wave_nordak_holy_creature();
            self.on_begin_wave_paulin_prompt_action();
            // TODO these can be converted to subskills
            self.on_begin_wave_space();
            self.on_begin_wave_marville();
        })
    }
}
