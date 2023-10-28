use derive_macro::PassiveSkill;

use crate::data::effect::Effect;
use crate::data::skill::Skill;
use crate::wave::for_ally_skill;
use crate::wave::for_any_skill;
use crate::wave::for_skill;
use crate::wave::has_skill;
use crate::wave::heroes::Cooldown;
use crate::wave::heroes::PassiveSkill;
use crate::wave::InstanceIndex;
use crate::wave::Wave;
use ordered_float::OrderedFloat;

#[derive(PassiveSkill, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct PromptAction {
    pub increase_self_turn_meter_ratio: f32,
    pub increase_ally_turn_meter_ratio: f32,
    pub increase_tenacity_ratio: f32,
    pub increase_tenacity_ratio_max: f32,
    pub effect_resistance_turns: u32,
    pub start_increase_turn_meter_ratio: f32,
}

impl Default for PromptAction {
    fn default() -> Self {
        Self {
            increase_self_turn_meter_ratio: 0.1,
            increase_ally_turn_meter_ratio: 0.1,
            increase_tenacity_ratio: 0.5,
            increase_tenacity_ratio_max: 0.63,
            effect_resistance_turns: 2,
            start_increase_turn_meter_ratio: 0.1,
        }
    }
}

impl Wave<'_> {
    pub fn on_begin_wave_paulin_prompt_action(&mut self) {
        for_any_skill!(
            self,
            Skill::PromptAction(PromptAction {
                effect_resistance_turns,
                increase_self_turn_meter_ratio,
                increase_ally_turn_meter_ratio,
                increase_tenacity_ratio,
                increase_tenacity_ratio_max,
                start_increase_turn_meter_ratio
            }),
            i,
            {
                if self.get_effect_resistance(i) < 1.0 && self.get_effect_resistance(i) > 0.5 {
                    self.inflict_ally_team(
                        i,
                        Effect::EffectResistanceUpI,
                        1.0,
                        effect_resistance_turns,
                    )
                }
                if self.get_effect_resistance(i) >= 1.0 {
                    self.inflict_ally_team(
                        i,
                        Effect::EffectResistanceUpII,
                        1.0,
                        effect_resistance_turns,
                    )
                }
                for e in self.get_enemies_indices(i) {
                    if self.get_speed(e) > self.get_speed(i) {
                        self.increase_turn_meter_ratio(i, i, start_increase_turn_meter_ratio);
                    }
                }
            }
        );
    }
}
