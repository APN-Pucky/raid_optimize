

use crate::wave::heroes::{Cooldown, Skilled, Typed, Selector, Execute};
use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill, SkillType, Select}, effect::{Effect}, }, };

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct Bondage {
    pub cooldown : u32,
    pub attack_damage_ratio : f32,
    pub second_attack_damage_ratio : f32,
    pub deep_poison_chance : f32,
    pub deep_poison_turns : u32,
}

impl Default for Bondage {
    fn default() -> Self {
        Self {
            cooldown : 3,
            attack_damage_ratio : 1.4,
            second_attack_damage_ratio : 2.0,
            deep_poison_chance : 1.0,
            deep_poison_turns : 2,
        }
    }
}

impl Execute for Bondage {
    fn execute(&self, wave : &mut Wave,  skill : &Skill, attacker:InstanceIndex, defender:InstanceIndex, ) {
       let actor = attacker;
       wave.attack_enemy_team(attacker, wave.get_attack_damage(attacker)  *self.attack_damage_ratio  , skill);
       wave.inflict_enemy_team(actor, Effect::DeepPoison, self.deep_poison_chance, self.deep_poison_turns);
       for i in wave.get_enemies_indices(actor) {
            let n = wave.effects[i].get(Effect::Poison);
            let scale = if n >5 {
                1.0 + 0.5
                }else {
                1.0 + 0.1*n as f32
                };
                // TODO "The additional DMG is treated as Poison DMG."
            wave.attack_single(attacker,i,wave.get_attack_damage(attacker) * self.second_attack_damage_ratio * scale,skill);
       }
    }
}

impl Typed for Bondage {
    const TYPE : SkillType = SkillType::Active;
}

impl Selector for Bondage {
    const SELECT : Select = Select::AllEnemies;
}

impl Skilled for Bondage {}