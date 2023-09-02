use crate::{wave::{Wave, InstanceIndex}, data::{skill::Skill, effect::Effect, }, };

impl<'a,const LEN:usize> Wave<'a,LEN> {
    pub fn execute_skill_liz(&mut self,  skill : &Skill, actor :InstanceIndex, target :InstanceIndex, ) {
        let attacker = actor;
        let defender = target;
        match skill {
            Skill::Resurrection { shield_max_hp_ratio, shield_turns, cleanse_dot_debuffs, restore_max_hp_ratio ,..} => {
                let max_hp = self.get_max_health(actor);
                self.restore_max_hp_ratio_own_team(actor,*restore_max_hp_ratio);
                self.shield_ally_team(actor,shield_max_hp_ratio * max_hp  ,*shield_turns);
                //TODO
                //self.cleanse_ally_team(actor,&is_dot,*cleanse_dot_debuffs);
            },
            Skill::FireHeal{heal_attack_ratio,heal_max_hp_ratio,block_debuff_turns,..} => {
                let heal = self.get_attack_damage(actor)*heal_attack_ratio ;
                let max_hp_heal = self.get_max_health(actor)*heal_max_hp_ratio ;
                self.restore(actor,target, heal + max_hp_heal);
                self.inflict_single(actor,target,Effect::BlockDebuff, 1.0,*block_debuff_turns);
            }
            Skill::ScorchedSoul{basic_attack,attack_damage_ratio,hp_burning_chance, hp_burning_turns ,..} => {
                self.attack_single(attacker, defender,  self.get_attack_damage(attacker)  *attack_damage_ratio, skill);
                self.inflict_single(attacker,defender,Effect::HPBurning, *hp_burning_chance, *hp_burning_turns);
                //self.inflict_hp_burning(attacker,defender, *hp_burning_chance, *hp_burning_turns);
            }
            _ => {}

        }
    }
}