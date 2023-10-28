use derive_macro::PassiveSkill;

use crate::data::effect::Effect;
use crate::data::skill::Skill;
use crate::wave::for_any_skill;
use crate::wave::for_skill;
use crate::wave::has_skill;
use crate::wave::heroes::Cooldown;
use crate::wave::heroes::PassiveSkill;
use crate::wave::InstanceIndex;
use crate::wave::Wave;
use ordered_float::OrderedFloat;

#[derive(PassiveSkill, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct HolyCreature {
    pub divine_dust_increase_shield: f32,
    pub divine_shield_max_health_ratio: f32,
    pub overflowing_light_alive_max_hp_ratio: f32,
    pub overflowing_light_dead_max_hp_ratio: f32,
    pub overflowing_light_turn_limit: u32,
}

impl Default for HolyCreature {
    fn default() -> Self {
        Self {
            divine_dust_increase_shield: 0.004,
            divine_shield_max_health_ratio: 0.1,
            overflowing_light_alive_max_hp_ratio: 0.35,
            overflowing_light_dead_max_hp_ratio: 0.15,
            overflowing_light_turn_limit: 3,
        }
    }
}

impl Wave<'_> {
    pub fn on_begin_wave_nordak_holy_creature(&mut self) {
        for_any_skill!(
            self,
            Skill::HolyCreature(HolyCreature {
                divine_dust_increase_shield,
                ..
            }),
            i,
            {
                self.inflict_ally_team(i, Effect::DivineShield, 1.0, 999);
                if self.is_alive(i) {
                    self.inflict_ally_team(i, Effect::OverflowingLight, 1.0, 999);
                }
            }
        );
    }

    pub fn on_turn_end_nordak_holy_creature(&mut self, actor: InstanceIndex) {
        for_any_skill!(
            self,
            Skill::HolyCreature(HolyCreature {
                divine_dust_increase_shield,
                ..
            }),
            i,
            {
                if self.is_alive(i) {
                    self.inflict_single(i, actor, Effect::DivineShield, 1.0, 999);
                }
            }
        );
    }

    pub fn on_inflicted_nordak_holy_creature(&mut self, actor: InstanceIndex, effect: Effect) {
        if effect == Effect::DivineShield {
            for_skill!(
                self,
                actor,
                Skill::HolyCreature(HolyCreature {
                    divine_dust_increase_shield,
                    ..
                }),
                {
                    self.inflict_single(actor, actor, Effect::DivineLight, 1.0, 999);
                }
            );
        }
    }
}
