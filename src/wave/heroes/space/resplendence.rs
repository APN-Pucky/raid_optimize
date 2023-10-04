use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill}, effect::{Effect}, }, };

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct Resplendence{
    pub turn_meter_ratio: f32,
}

impl Default for Resplendence{
    fn default() -> Self {
        Self {
            turn_meter_ratio: 0.,
        }
    }
}
