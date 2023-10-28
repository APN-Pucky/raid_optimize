use crate::data::effect::Effect;
use crate::wave::heroes::Cooldown;
use crate::{
    data::skill::{Select, Skill, SkillType},
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct ThunderPalm {
    pub cooldown: u32,
    pub defense_damage_ratio: f32,
    pub stun_chance: f32,
    pub stun_turns: u32,
    pub stun_chance_per_target_buff: f32,
}

impl Default for ThunderPalm {
    fn default() -> Self {
        Self {
            cooldown: 0,
            defense_damage_ratio: 2.2,
            stun_chance: 0.25,
            stun_turns: 1,
            stun_chance_per_target_buff: 0.03,
        }
    }
}

impl ThunderPalm {
    pub const TYPE: SkillType = SkillType::Basic;
    pub const SELECT: Select = Select::SingleEnemy;

    pub fn execute(
        &self,
        wave: &mut Wave,
        skill: &Skill,
        actor: InstanceIndex,
        defender: InstanceIndex,
    ) {
        let _dmg_dealt = wave.attack_single(
            actor,
            defender,
            self.defense_damage_ratio * wave.get_defense(actor),
            skill,
        );
        // Get number of buffs on target
        let buffs = wave.count_buffs_layers(defender);
        wave.inflict_single(
            actor,
            defender,
            Effect::Stun,
            self.stun_chance + self.stun_chance_per_target_buff * buffs as f32,
            self.stun_turns,
        );
    }
}
