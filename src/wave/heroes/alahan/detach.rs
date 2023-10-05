use crate::wave::heroes::{Cooldown, Skilled, Typed, Selector, Execute};
use crate::{wave::{Wave, InstanceIndex,  }, data::{skill::{Skill, Select, SkillType}, effect::{Effect}, }, };

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct Detach {
    pub cooldown : u32,
    pub attack_damage_ratio : f32,
    pub stun_chance: f32,
    pub stun_turns: u32,
    pub steal_shield: bool,
    pub shield_max_hp_ratio: f32,
    pub shield_turns: u32,
}

impl Default for Detach {
    fn default() -> Self {
        Self {
            cooldown : 4,
            attack_damage_ratio : 1.6,
            stun_chance: 0.4,
            stun_turns: 1,
            steal_shield: true,
            shield_max_hp_ratio: 0.2,
            shield_turns: 2,
        }
    }
}

impl Detach {
    pub const TYPE : SkillType = SkillType::Active;
    pub const SELECT : Select = Select::AllEnemies;

    pub fn execute(&self, wave : &mut Wave,  skill : &Skill, actor:InstanceIndex, defender:InstanceIndex, ) {
                // remove all enemies shields
                for e in wave.get_enemies_indices(actor) {
                    if self.steal_shield {
                        wave.steal_shield(actor, e);
                    }
                    else {
                        wave.clear_shield(actor, e);
                    }
                }
                wave.attack_enemy_team(actor,self. attack_damage_ratio* wave.get_attack_damage(actor), skill);
                wave.inflict_enemy_team(actor, Effect::Stun, self.stun_chance, self.stun_turns);
                wave.shield_ally_team(actor, wave.get_max_health(actor) *self.shield_max_hp_ratio, self.shield_turns);
    }
}

