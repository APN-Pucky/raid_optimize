use crate::data::effect::is_dot;
use crate::wave::heroes::{Cooldown};
use crate::{wave::{Wave, InstanceIndex,  }, data::{skill::{Skill, Select, SkillType}, effect::{Effect}, }, };

use rand::seq::SliceRandom;

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct LightOfPurifying {
    pub cooldown : u32,
    pub heal_allies : u32,
    pub max_hp_restore_ratio : f32,
    pub heal_turns : u32,
    pub cleanse_dot_layers: u32,
}

impl Default for LightOfPurifying {
    fn default() -> Self {
        Self {
            cooldown : 3,
            heal_allies : 2,
            max_hp_restore_ratio : 0.15,
            heal_turns : 2,
            cleanse_dot_layers: 3,
        }
    }
}

impl LightOfPurifying {
    pub const TYPE : SkillType = SkillType::Active;
    pub const SELECT : Select = Select::AllAllies;

    pub fn execute(&self, wave : &mut Wave,  _skill : &Skill, actor:InstanceIndex, target:InstanceIndex, ) {
                let t =  target;
                if wave.health[t] == wave.get_max_health(t) {
                    wave.cleanse(target, &is_dot, self.cleanse_dot_layers);
                    wave.inflict_single(actor, target, Effect::Heal, 1.0, self.heal_turns);
                }
                wave.restore_single(actor, target, self.max_hp_restore_ratio* wave.get_max_health(actor));
                // also heal 2 random allies
                let mut allies = wave.get_ally_indices(actor);
                let mut rng = rand::thread_rng();
                allies.shuffle(&mut rng);
                let mut n = 0;
                while n < self.heal_allies {
                    if let Some(i) = allies.pop() {
                        if i != target {
                            wave.restore_single(actor, i, self.max_hp_restore_ratio* wave.get_max_health(actor));
                        }
                    }
                    n += 1;
                }
    }
}

