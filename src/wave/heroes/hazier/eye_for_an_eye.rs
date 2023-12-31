use crate::wave::heroes::Cooldown;
use crate::{
    data::{
        effect::Effect,
        skill::{Select, Skill, SkillType},
    },
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct EyeForAnEye {
    pub cooldown: u32,
    pub counter_attack_turns: u32,
    pub damage_immunity_turns: u32,
    pub control_immunity_turns: u32,
}

impl Default for EyeForAnEye {
    fn default() -> Self {
        Self {
            cooldown: 4,
            counter_attack_turns: 3,
            damage_immunity_turns: 2,
            control_immunity_turns: 2,
        }
    }
}

impl EyeForAnEye {
    pub const TYPE: SkillType = SkillType::Active;
    pub const SELECT: Select = Select::AllEnemies;

    pub fn execute(
        &self,
        wave: &mut Wave,
        skill: &Skill,
        actor: InstanceIndex,
        _target: InstanceIndex,
    ) {
        wave.inflict_single(
            actor,
            actor,
            Effect::Counterattack,
            1.0,
            self.counter_attack_turns,
        );
        wave.inflict_single(
            actor,
            actor,
            Effect::DamageImmunity,
            1.0,
            self.damage_immunity_turns,
        );
        wave.inflict_single(
            actor,
            actor,
            Effect::ControlImmunity,
            1.0,
            self.control_immunity_turns,
        );

        wave.act(actor);
    }
}
