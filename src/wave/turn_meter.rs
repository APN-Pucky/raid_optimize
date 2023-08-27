use crate::{indent,debug, hero::effect::Effect};

use super::{Wave, InstanceIndex};

impl<const LEN:usize> Wave<'_,LEN> {
    pub fn get_turn_meter(&self, actor : InstanceIndex) -> f32 {
        self.turn_meter[actor]
    }

    pub fn set_turn_meter(&mut self, actor : InstanceIndex, turn_meter : f32) {
        self.turn_meter[actor] = turn_meter;
    }

    pub fn increase_turn_meter(&mut self, actor : InstanceIndex, turn_meter: f32) {
        self.turn_meter[actor] += turn_meter;
    }

    pub fn reduce_turn_meter(&mut self, actor : InstanceIndex, target: InstanceIndex, turn_meter: f32) {
        let turn_meter = turn_meter * (1.0 - self.get_turn_meter_reduction_reduction(target));
        debug!("{} turn_meter reduced by {} from {} to {}", self.name(target),self.name(actor), turn_meter, self.turn_meter[target] - turn_meter);
        self.turn_meter[target] = (self.turn_meter[target]-turn_meter).max(0.0);
    }

    pub fn progress_turn_meter(&mut self, actor : InstanceIndex, time : f32) {
        self.turn_meter[actor] += self.get_speed(actor)*time;
        debug!("{} turn_meter progressed to {}", self.name(actor), self.turn_meter[actor]);
    }

    pub fn increase_turn_meter_team(&mut self, actor : InstanceIndex, increase_ratio : f32) {
        self.get_ally_indices(actor).iter()
            .for_each(|&i| 
                self.increase_turn_meter(i,increase_ratio * self.turn_meter_threshold)
            )
        //(0..LEN)
        //    .filter(|i| self.teams[*i] != self.teams[actor])
        //    .for_each(|i| 
        //        self.increase_turn_meter(i,increase_ratio * self.turn_meter_threshold)
        //    )
    }


    pub fn progress_all_turn_meters(&mut self) {
        debug!("Progressing turn meters");
        indent!({
            // get the time needed for one to reach threshold
            let mut min : f32 = (0..self.len)
                .filter(|a| self.is_alive(*a))
                .map(|a| (self.turn_meter_threshold - self.get_turn_meter(a) )/(self.get_speed(a)))
                .min_by(|a,b| a.partial_cmp(b).unwrap()).unwrap();
                //.reduce( |a, b| a.min(b)).unwrap();
            if min < 0.0 {
                min = 0.0;
            }
            for a in 0..self.len {
                self.progress_turn_meter(a,min);
            }
        })
    }
}