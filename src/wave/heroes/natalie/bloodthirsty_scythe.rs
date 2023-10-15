// TODO needs Exclusive
use crate::wave::heroes::Cooldown;
use crate::{
    data::{
        effect::Effect,
        skill::{Select, Skill, SkillType},
    },
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct BloodthirstyScythe {
    pub cooldown: u32,
    pub attack_damage_ratio: f32,
    pub bleed_chance: f32,
    pub bleed_turns: u32,
}

impl Default for BloodthirstyScythe {
    fn default() -> Self {
        Self {
            cooldown: 3,
            attack_damage_ratio: 1.4,
            bleed_chance: 0.8,
            bleed_turns: 2,
        }
    }
}

impl BloodthirstyScythe {
    pub const TYPE: SkillType = SkillType::Active;
    pub const SELECT: Select = Select::AllEnemies;

    pub fn execute(
        &self,
        wave: &mut Wave,
        skill: &Skill,
        actor: InstanceIndex,
        _defender: InstanceIndex,
    ) {
        let damage = wave.get_attack_damage(actor) * self.attack_damage_ratio;
        wave.attack_enemy_team(actor, damage, skill);
        wave.inflict_enemy_team(actor, Effect::Bleed, self.bleed_chance, self.bleed_turns);
    }
}
