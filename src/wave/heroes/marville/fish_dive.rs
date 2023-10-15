use crate::wave::heroes::Cooldown;
use crate::{
    data::{
        effect::Effect,
        skill::{Select, Skill, SkillType},
    },
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct FishDive {
    pub cooldown: u32,
    pub restore_fish_shoal: u32,
}

impl Default for FishDive {
    fn default() -> Self {
        Self {
            cooldown: 4,
            restore_fish_shoal: 3,
        }
    }
}
impl FishDive {
    pub const TYPE: SkillType = SkillType::Active;
    pub const SELECT: Select = Select::AllAllies;

    pub fn execute(
        &self,
        wave: &mut Wave,
        _skill: &Skill,
        actor: InstanceIndex,
        _target: InstanceIndex,
    ) {
        wave.restore_to_highest_ally_health_percentage(actor);
        for i in wave.get_ally_indices(actor) {
            if wave.has_effect(i, Effect::FishShoal) {
                wave.reduce_cooldowns(i);
            }
        }
        for _ in 0..self.restore_fish_shoal {
            wave.inflict_ally_team(actor, Effect::FishShoal, 1.0, 999)
        }
    }
}
