use derive_macro::Cooldown;

use crate::wave::heroes::{Cooldown, Skilled, Typed, Selector, Execute};
use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill, Select, SkillType}, effect::{Effect, is_attribute_debuff}, }, };

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct SpiritFountain{
    pub cooldown : u32,
    pub heal_turns: u32,
    pub cleanse_attribute_debuffs: bool,
}

impl Default for SpiritFountain{
    fn default() -> Self {
        Self {
            cooldown : 3,
            heal_turns : 2,
            cleanse_attribute_debuffs : true,
        }
    }
}

impl SpiritFountain{
    pub const TYPE : SkillType = SkillType::Active;
    pub const SELECT : Select = Select::AllAllies;

    pub fn execute(&self, wave : &mut Wave,  skill : &Skill, actor:InstanceIndex, defender:InstanceIndex, ) {
                wave.restore_to_highest_ally_health_percentage(actor);
                if self.cleanse_attribute_debuffs {
                    wave.cleanse_team(actor, &is_attribute_debuff, 999)
                }
                wave.inflict_ally_team(actor, Effect::Heal, 1.0, self.heal_turns);
    }
}

