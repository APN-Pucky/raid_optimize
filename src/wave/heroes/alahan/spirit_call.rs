use crate::wave::heroes::Cooldown;
use crate::{
    data::skill::{Select, Skill, SkillType},
    wave::{InstanceIndex, Wave},
};

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub struct SpiritCall {
    pub cooldown: u32,
    pub attack_damage_ratio: f32,
    pub restore_hp_damage_ratio: f32,
    pub remove_all_buffs: bool,
    pub heal_lowest_ally: bool,
    pub increase_hp: bool,
}

impl Default for SpiritCall {
    fn default() -> Self {
        Self {
            cooldown: 0,
            attack_damage_ratio: 1.8,
            restore_hp_damage_ratio: 0.6,
            remove_all_buffs: true,
            heal_lowest_ally: true,
            increase_hp: true,
        }
    }
}

impl SpiritCall {
    pub const TYPE: SkillType = SkillType::Basic;
    pub const SELECT: Select = Select::SingleEnemy;

    pub fn execute(
        &self,
        wave: &mut Wave,
        _skill: &Skill,
        actor: InstanceIndex,
        target: InstanceIndex,
    ) {
        let n = wave.get_number_of_buff_layers(target).min(5);
        if self.remove_all_buffs {
            wave.remove_all_buffs_single(actor, target);
        } else {
            panic!("Not implemented");
        }
        wave.attack_single(
            actor,
            target,
            wave.get_attack_damage(actor) * self.attack_damage_ratio,
            &Skill::SpiritCall(SpiritCall {
                cooldown: self.cooldown,
                attack_damage_ratio: self.attack_damage_ratio,
                restore_hp_damage_ratio: self.restore_hp_damage_ratio + 0.2 * n as f32,
                remove_all_buffs: self.remove_all_buffs,
                heal_lowest_ally: self.heal_lowest_ally,
                increase_hp: self.increase_hp,
            }),
        );
    }
}
