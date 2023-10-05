
use crate::wave::heroes::{Cooldown, Skilled, Typed, Selector, Execute};
use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill, SkillType, Select}, effect::{Effect}, }, };

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct DeepSeaPower {
    pub cooldown : u32,
    pub max_hp_shield_ratio : f32,
    pub shield_turns : u32,
    pub tenacity_increase_turns : u32,
}

impl Default for DeepSeaPower{
    fn default() -> Self {
        Self {
            cooldown : 5,
            max_hp_shield_ratio : 0.25,
            shield_turns : 2,
            tenacity_increase_turns : 2,
        }
    }
}

impl DeepSeaPower{
    pub const TYPE : SkillType = SkillType::Active;
    pub const SELECT : Select = Select::AllEnemies;
    pub fn execute(&self, wave : &mut Wave,  skill : &Skill, actor:InstanceIndex, defender:InstanceIndex, ) {
        let max_hp = wave.get_max_health(actor);
        wave.shield_ally_team(actor,self.max_hp_shield_ratio * max_hp  ,self.shield_turns);
        wave.inflict_ally_team(actor, Effect::TenacityUpII, 1.0, self.tenacity_increase_turns);
    }
}
