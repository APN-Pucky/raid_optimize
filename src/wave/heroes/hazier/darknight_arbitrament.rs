

use crate::wave::heroes::{Cooldown};
use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill, SkillType, Select}, effect::{Effect, is_buff}, }, debug, indent, };

#[derive(Cooldown,Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct DarknightArbitrament{
        pub cooldown : u32,
        pub attack_damage_ratio : f32,
        pub crit_rate_turns : u32,
        pub crit_damage_turns : u32,
}

impl Default for DarknightArbitrament {
    fn default() -> Self {
        Self {
            cooldown : 4,
            attack_damage_ratio : 5.,
            crit_rate_turns : 2,
            crit_damage_turns : 2,
        }
    }
}

impl DarknightArbitrament{
    pub const TYPE : SkillType = SkillType::Active;
    pub const SELECT : Select = Select::AllEnemies;

    pub fn execute(&self, wave : &mut Wave,  skill : &Skill, actor:InstanceIndex, target:InstanceIndex, ) {
        wave.inflict_single(actor,actor,Effect::CritRateUpI,1.0,self.crit_rate_turns);
        wave.inflict_single(actor,actor,Effect::CritDamageUpI,1.0,self.crit_damage_turns);
        // copy buffs from defender to wave
        debug!("Copying buffs from {:?} to {:?}",wave.name(target),wave.name(actor));
        indent!({
            //cloned loop to allow copying of buffs from wave to wave, which should never happen
            for effect in wave.effects[target].em.iter().map(|(effect,_)| effect).filter(|effect| is_buff(*effect)).collect::<Vec<_>>() {
                let cloned_vec: Vec<_> = wave.effects[target].em[effect].iter().cloned().collect();
                wave.effects[actor].em[effect].extend(cloned_vec);
            }
        });
        wave.attack_single(actor,target, wave.get_attack_damage(actor) * self.attack_damage_ratio, skill);
    }
}
