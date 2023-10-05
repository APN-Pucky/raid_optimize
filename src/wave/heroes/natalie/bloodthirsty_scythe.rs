
use crate::wave::heroes::Cooldown;
use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill, SkillType, Select}, effect::{Effect}, }, };

#[derive(Cooldown,Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct BloodthirstyScythe{
    pub cooldown : u32,
    pub attack_damage_ratio : f32,
    pub bleed_chance: f32,
    pub bleed_turns: u32,
}

impl Default for BloodthirstyScythe{
    fn default() -> Self {
        Self {
                cooldown : 4,
                attack_damage_ratio : 1.4,
                bleed_chance: 0.5,
                bleed_turns: 2,
        }
    }
}

impl BloodthirstyScythe{

    pub const TYPE : SkillType = SkillType::Active;
    pub const SELECT : Select = Select::SingleEnemy;

    pub fn execute(&self, wave : &mut Wave,  skill : &Skill, actor:InstanceIndex, defender:InstanceIndex, ) {
        let damage = wave.get_attack_damage(actor) * self.attack_damage_ratio;
        wave.attack_enemy_team(actor, damage ,skill);
        wave.inflict_enemy_team(actor, Effect::Bleed, self.bleed_chance, self.bleed_turns);
    }
}
