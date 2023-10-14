use crate::data::effect::is_attribute_debuff;
use crate::roll;
use crate::wave::heroes::{Cooldown};
use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill, SkillType, Select}, }, };

#[derive(Cooldown,Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct FishWaterball {
    pub cooldown : u32,
    pub attack_damage_ratio : f32,
    pub act_chance: f32,
}

impl Default for FishWaterball{
    fn default() -> Self {
        Self {
            cooldown : 0,
            attack_damage_ratio : 1.2,
            act_chance: 0.3,
        }
    }
}

impl FishWaterball {
    pub const TYPE : SkillType = SkillType::Basic;
    pub const SELECT: Select= Select::SingleEnemy;

    pub fn execute(&self, wave : &mut Wave,  skill : &Skill, actor:InstanceIndex, _target:InstanceIndex, ) {
        let damage = wave.get_attack_damage(actor) * self.attack_damage_ratio;
        wave.attack_enemy_team(actor, damage ,skill);
        wave.refresh_enemy_team(actor, &is_attribute_debuff);
        if roll(self.act_chance){
            // take another turn
            wave.act(actor);
        }
    }
}