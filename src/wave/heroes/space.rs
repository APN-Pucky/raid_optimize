use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill, SkillData}, effect::Effect, }, };

impl<'a,const LEN:usize> Wave<'a,LEN> {
    pub fn execute_skill_space(&mut self,  skill : &Skill, actor :InstanceIndex, target :InstanceIndex, ) {
        let attacker = actor;
        let defender = target;
        match skill.data {
            SkillData::Tricks{attack_damage_ratio,turn_meter_reduction_ratio: turn_meter_reduction,..} => {
                self.attack_single(attacker,defender, (self.get_attack_damage(attacker) * attack_damage_ratio), skill);
                self.reduce_turn_meter(attacker,defender, turn_meter_reduction);
            },
            SkillData::Nightmare {   attack_damage_ratio, reduce_speed_chance, reduce_speed_turns, increase_speed_turns ,..} => {
                self.attack_single(attacker,defender, (self.get_attack_damage(attacker) * attack_damage_ratio),skill);
                self.inflict_single(attacker,defender,Effect::SpeedDownII, reduce_speed_chance, reduce_speed_turns);
                self.inflict_ally_team(actor, Effect::SpeedUpI, 1.0,increase_speed_turns);
                //TODO target make no sense here
                //attacker.inflict(defender,Effect::SpeedUpI, 1.0, increase_speed_turns);

            },
            SkillData::FissionOfLife {  restore_max_hp_ratio, heal_turns, increase_turn_meter_ratio ,..} => {
                self.restore_max_hp_ratio_own_team(actor, restore_max_hp_ratio);
                self.inflict_ally_team(actor, Effect::Heal,1.0, heal_turns);
                self.increase_turn_meter_team(actor, increase_turn_meter_ratio);
            },
            _ => {}

        }
    }
}