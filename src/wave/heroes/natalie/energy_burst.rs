
use crate::wave::heroes::{Cooldown, Skilled, Typed, Selector, Execute};
use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill, SkillType, Select}, effect::{Effect}, }, };

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct EnergyBurst{
    pub cooldown : u32,
    pub attack_damage_ratio : f32,
    pub bleed_turns: u32,
    pub reduce_effect_resistance_chance :f32,
    pub reduce_effect_resistance_turns : u32
}

impl Default for EnergyBurst{
    fn default() -> Self {
        Self {
            cooldown :4,
            attack_damage_ratio : 1.4,
            bleed_turns : 2,
            reduce_effect_resistance_chance : 0.3,
            reduce_effect_resistance_turns : 2
        }
    }
}

impl EnergyBurst{
    pub const TYPE : SkillType = SkillType::Active;
    pub const SELECT : Select = Select::AllEnemies;
    pub fn execute(&self, wave : &mut Wave,  skill : &Skill, actor:InstanceIndex, defender:InstanceIndex, ) {
        let damage = wave.get_attack_damage(actor) * self.attack_damage_ratio;
        wave.attack_enemy_team(actor, damage ,skill);
        wave.inflict_enemy_team(actor, Effect::Bleed, 1.0, self.bleed_turns);
        wave.inflict_enemy_team(actor, Effect::EffectResistanceDownII, self.reduce_effect_resistance_chance, self.reduce_effect_resistance_turns);
    }
}
