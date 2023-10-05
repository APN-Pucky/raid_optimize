use crate::{wave::{Wave, InstanceIndex, heroes::{PassiveSkill, Cooldown, Selector, Typed, Execute, Skilled}, }, data::{skill::{Skill, SkillType, Select, get_cooldown}, effect::{Effect}, }, };


#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct SoulSeal {
    pub cooldown : u32,
    pub attack_damage_ratio : f32,
    pub attack_damage_ratio_per_poison : f32,
    pub increase_atk_turns : u32,
    pub rose_per_poison : u32,
    pub poison_turns : u32
}

impl Default for SoulSeal {
    fn default() -> Self {
        Self {
            cooldown : 3,
            attack_damage_ratio : 4.,
            attack_damage_ratio_per_poison : 1.2,
            increase_atk_turns : 2,
            rose_per_poison : 4,
            poison_turns : 2
        }
    }
}

impl Selector for SoulSeal {
    const SELECT :Select = Select::AllEnemies;
}
impl Typed for SoulSeal {
    const TYPE : SkillType  = SkillType::Active; 
}
impl Execute for SoulSeal {
    fn execute(&self, wave : &mut crate::wave::Wave<'_>, skill : &crate::data::skill::Skill, actor : crate::wave::InstanceIndex, target : crate::wave::InstanceIndex) {
                wave.inflict_ally_team(actor, Effect::AttackUpII, 1.0, self.increase_atk_turns);
                // counter number of effects arcane
                let poison = wave.effects[target].get(Effect::Poison);
                let _rose_poison = wave.effects[target].get(Effect::RosePoison);

                wave.effects[target].clear_single(Effect::Poison);
                wave.attack_single(actor, target, wave.get_attack_damage(actor) * (self.attack_damage_ratio + poison as f32 * self.attack_damage_ratio_per_poison), skill);

                while wave.effects[target].get(Effect::RosePoison) >= self.rose_per_poison {
                    wave.inflict_single(actor, target, Effect::Poison, 1.0, self.poison_turns );
                    wave.effects[target].remove_layers(Effect::RosePoison,self.rose_per_poison);
                }
    }
}
impl Skilled for SoulSeal {}