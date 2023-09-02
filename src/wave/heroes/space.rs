use crate::{wave::{Wave, InstanceIndex}, data::{skill::Skill, effect::Effect, effects::{buff::{Buff, attribute::AttributeBuff}, debuff::attribute::AttributeDebuff}}};

impl<'a,const LEN:usize> Wave<'a,LEN> {
    pub fn execute_skill_space(&mut self,  skill : &Skill, actor :InstanceIndex, target :InstanceIndex, ) {
        let attacker = actor;
        let defender = target;
        match skill {
            Skill::Tricks{basic_attack,attack_damage_ratio,turn_meter_reduction_ratio: turn_meter_reduction,..} => {
                self.attack_single(attacker,defender, (self.get_attack_damage(attacker) * attack_damage_ratio), skill);
                self.reduce_turn_meter(attacker,defender, *turn_meter_reduction);
            },
            Skill::Nightmare { basic_attack,  attack_damage_ratio, reduce_speed_chance, reduce_speed_turns, increase_speed_turns ,..} => {
                self.attack_single(attacker,defender, (self.get_attack_damage(attacker) * attack_damage_ratio),skill);
                self.inflict_attribute_debuff_single(attacker,defender,AttributeDebuff::SpeedDownII, *reduce_speed_chance, *reduce_speed_turns);
                self.inflict_attribute_buff_ally_team(actor, AttributeBuff::SpeedUpI, *increase_speed_turns);
                //TODO target make no sense here
                //attacker.inflict(defender,Effect::SpeedUpI, 1.0, increase_speed_turns);

            },
            Skill::FissionOfLife {  restore_max_hp_ratio, heal_turns, increase_turn_meter_ratio ,..} => {
                self.restore_max_hp_ratio_own_team(actor, *restore_max_hp_ratio);
                self.inflict_buff_ally_team(actor, Buff::Heal, *heal_turns);
                self.increase_turn_meter_team(actor, *increase_turn_meter_ratio);
            },
            _ => {}

        }
    }
}