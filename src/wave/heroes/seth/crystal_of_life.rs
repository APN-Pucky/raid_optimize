use crate::wave::heroes::Cooldown;
use crate::{
    data::{
        effect::Effect,
        skill::{Select, Skill, SkillType},
    },
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct CrystalOfLife {
    pub cooldown: u32,
    pub max_hp_restore_ratio: f32,
    pub ripple_turns: u32,
    pub attack_up_turns: u32,
}

impl Default for CrystalOfLife {
    fn default() -> Self {
        Self {
            cooldown: 5,
            max_hp_restore_ratio: 0.22,
            ripple_turns: 2,
            attack_up_turns: 2,
        }
    }
}

impl CrystalOfLife {
    pub const TYPE: SkillType = SkillType::Active;
    pub const SELECT: Select = Select::AllEnemies;
    pub fn execute(
        &self,
        wave: &mut Wave,
        _skill: &Skill,
        actor: InstanceIndex,
        _defender: InstanceIndex,
    ) {
        let rest_hp = wave.get_max_health(actor) * self.max_hp_restore_ratio;
        wave.restore_ally_team(actor, rest_hp);
        wave.inflict_ally_team(actor, Effect::RippleII, 1.0, self.ripple_turns);
        wave.inflict_ally_team(actor, Effect::AttackUpII, 1.0, self.attack_up_turns);
    }
}
