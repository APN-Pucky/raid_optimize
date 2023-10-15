use crate::wave::heroes::{Cooldown, Execute, Selector, Skilled, Typed};
use crate::{
    data::{
        effect::Effect,
        skill::{Select, Skill, SkillType},
    },
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct LeavesStorm {
    pub cooldown: u32,
    pub attack_damage_ratio: f32,
}

impl Default for LeavesStorm {
    fn default() -> Self {
        Self {
            cooldown: 3,
            attack_damage_ratio: 1.85,
        }
    }
}

impl Execute for LeavesStorm {
    fn execute(
        &self,
        wave: &mut Wave,
        skill: &Skill,
        attacker: InstanceIndex,
        _defender: InstanceIndex,
    ) {
        wave.attack_enemy_team(
            attacker,
            wave.get_attack_damage(attacker)
                * self.attack_damage_ratio
                * (1. + 0.02 * wave.effects[attacker].get(Effect::ScarletSakura) as f32),
            skill,
        );
    }
}

impl Typed for LeavesStorm {
    const TYPE: SkillType = SkillType::Active;
}

impl Selector for LeavesStorm {
    const SELECT: Select = Select::AllEnemies;
}

impl Skilled for LeavesStorm {}
