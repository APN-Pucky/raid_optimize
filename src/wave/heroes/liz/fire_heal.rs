use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill}, effect::{Effect}, }, };

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct FireHeal{
    pub heal_attack_ratio: f32,
    pub heal_max_hp_ratio: f32,
    pub block_debuff_turns: u32,
}

impl Default for FireHeal {
    fn default() -> Self {
        Self {
            heal_attack_ratio: 0.8,
            heal_max_hp_ratio: 0.06,
            block_debuff_turns: 2,
        }
    }
}

impl FireHeal{
    pub fn execute(&self, wave : &mut Wave,  skill : &Skill, actor:InstanceIndex, target:InstanceIndex, ) {
        let heal = wave.get_attack_damage(actor)*self.heal_attack_ratio ;
        let max_hp_heal = wave.get_max_health(actor)*self.heal_max_hp_ratio ;
        wave.restore(actor,target, heal + max_hp_heal);
        wave.inflict_single(actor,target,Effect::BlockDebuff, 1.0,self.block_debuff_turns);
    }
}
