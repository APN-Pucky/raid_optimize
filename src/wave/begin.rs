use crate::{indent, debug};

use super::Wave;

impl Wave<'_> {
    pub fn begin(&mut self) {
        debug!("Wave begin");
        indent!({
            self.on_begin_wave_space();
            self.on_begin_wave_marville();
        })
    }
}