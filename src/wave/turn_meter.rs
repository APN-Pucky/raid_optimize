use crate::{debug, indent, data::effect::Effect};

use super::{InstanceIndex, Wave};

impl Wave<'_> {
    pub fn get_turn_meter(&self, actor: InstanceIndex) -> f32 {
        self.turn_meter[actor]
    }

    pub fn set_turn_meter(&mut self, actor: InstanceIndex, turn_meter: f32) {
        self.turn_meter[actor] = turn_meter;
    }

    pub fn steal_turn_meter_ratio(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        turn_meter_ratio: f32,
    ) {
        let stolen = self.reduce_turn_meter_ratio(actor, target, turn_meter_ratio);
        self.increase_turn_meter(actor, target, stolen)
    }

    pub fn increase_turn_meter_ratio(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        turn_meter_ratio: f32,
    ) {
        self.increase_turn_meter(actor, target, turn_meter_ratio * self.turn_meter_threshold);
    }

    pub fn increase_turn_meter(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        turn_meter: f32,
    ) {
        if self.has_effect(actor, Effect::Imprison) {
            debug!("{} is imprisoned -> can't increase turn meter", self.name(actor));
            return;
        }
        self.turn_meter[target] = self
            .turn_meter_threshold
            .min(self.turn_meter[target] + turn_meter);
        debug!(
            "{} increases {}'s turn_meter by {} to {}",
            self.name(actor),
            self.name(target),
            turn_meter,
            self.turn_meter[target]
        );
    }

    pub fn reduce_turn_meter_ratio(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        turn_meter_ratio: f32,
    ) -> f32 {
        self.reduce_turn_meter(actor, target, turn_meter_ratio * self.turn_meter_threshold)
    }

    pub fn reduce_turn_meter(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        turn_meter: f32,
    ) -> f32 {
        let turn_meter = turn_meter * (1.0 - self.get_turn_meter_reduction_reduction(target));
        debug!(
            "{} turn_meter reduced by {} from {} to {}",
            self.name(target),
            self.name(actor),
            self.turn_meter[target],
            self.turn_meter[target] - turn_meter
        );
        let old = self.turn_meter[target];
        self.turn_meter[target] = (self.turn_meter[target] - turn_meter).max(0.0);
        old - self.turn_meter[target]
    }

    pub fn progress_turn_meter(&mut self, actor: InstanceIndex, time: f32) {
        self.turn_meter[actor] += self.get_speed(actor) * time;
        debug!(
            "{} turn_meter progressed to {}",
            self.name(actor),
            self.turn_meter[actor]
        );
    }

    pub fn increase_turn_meter_team(&mut self, actor: InstanceIndex, increase_ratio: f32) {
        debug!("{} increases turn meter of team", self.name(actor));
        indent!({
            self.get_ally_indices(actor).iter().for_each(|&i| {
                self.increase_turn_meter(actor, i, increase_ratio * self.turn_meter_threshold)
            })
        });
    }

    pub fn progress_all_turn_meters(&mut self) {
        debug!("Progressing turn meters");
        indent!({
            // get the time needed for one to reach threshold
            let mut min: f32 = self
                .get_indices_iter()
                .filter(|a| self.is_alive(*a))
                .map(|a| (self.turn_meter_threshold - self.get_turn_meter(a)) / (self.get_speed(a)))
                .fold(self.turn_meter_threshold, |a, b| a.min(b));
            //.min_by(|a,b| a.partial_cmp(b).unwrap()).unwrap();
            //.reduce( |a, b| a.min(b)).unwrap();
            if min < 0.0 {
                min = 0.0;
            }
            for a in self.get_indices_iter() {
                self.progress_turn_meter(a, min);
            }
        })
    }
}
