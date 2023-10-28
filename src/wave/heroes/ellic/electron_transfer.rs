use derive_macro::PassiveSkill;

use crate::data::skill::Skill;
use crate::wave::for_skill;
use crate::wave::has_skill;
use crate::wave::heroes::Cooldown;
use crate::wave::heroes::PassiveSkill;
use crate::wave::InstanceIndex;
use crate::wave::Wave;
use ordered_float::OrderedFloat;

#[derive(PassiveSkill, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct ElectronTransfer {
    pub crit_damage_reduction: f32,
    pub steal_buff_layers: u32,
    pub max_transfers: u32,
}

impl Default for ElectronTransfer {
    fn default() -> Self {
        Self {
            crit_damage_reduction: 0.4,
            steal_buff_layers: 1,
            max_transfers: 3,
        }
    }
}

impl Wave<'_> {
    pub fn on_turn_start_ellic_electron_transfer(&mut self, i: InstanceIndex) {
        for_skill!(
            self,
            i,
            Skill::ElectronTransfer(ElectronTransfer {
                steal_buff_layers,
                ..
            }),
            {
                if self.is_alive(i) {
                    if let Some(j) = self.find_highest_attack_alive_enemy(i) {
                        self.steal_random_buff_layers(i, j, steal_buff_layers)
                    }
                }
            }
        );
    }
}
