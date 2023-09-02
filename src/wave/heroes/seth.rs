use crate::{wave::{Wave, InstanceIndex}, data::{skill::Skill, effect::Effect, }, };

impl<'a,const LEN:usize> Wave<'a,LEN> {
    pub fn execute_skill_seth(&mut self,  skill : &Skill, actor :InstanceIndex, target :InstanceIndex, ) {
        let attacker = actor;
        let defender = target;
        match skill {

            Skill::DeepSeaPower {  max_hp_shield_ratio, shield_turns, tenacity_increase_turns ,..} => {
                let max_hp = self.get_max_health(actor);
                self.shield_ally_team(actor,max_hp_shield_ratio * max_hp  ,*shield_turns);
                self.inflict_ally_team(actor, Effect::TenacityUpII, 1.0, *tenacity_increase_turns);
            },
            Skill::CrystalOfLife {  max_hp_restore_ratio, ripple_turns , attack_up_turns ,..} =>{
                let rest_hp = (self.get_max_health(actor)  * max_hp_restore_ratio) ;
                self.restore_ally_team(actor,rest_hp);
                self.inflict_ally_team(actor, Effect::RippleII, 1.0, *ripple_turns);
                self.inflict_ally_team(actor, Effect::AttackUpII, 1.0,* attack_up_turns);
            },
            Skill::TideBigHit {basic_attack, max_hp_damage_ratio, suffocated_chance, suffocated_turns,.. } => {
                log::debug!("{} uses Tide Big Hit on {}", attacker, defender);
                let mut chance = *suffocated_chance;
                self.attack_single(attacker,defender, (self.get_max_health(attacker) * max_hp_damage_ratio), skill);
                if self.has_effect(defender,Effect::WetI) 
                || self.has_effect(defender,Effect::WetII) 
                || self.has_effect(defender,Effect::ColdI) 
                || self.has_effect(defender,Effect::ColdII){
                    log::debug!("{} is wet or cold +15% suffocation chance", defender);
                    chance += 0.15;
                }
                self.inflict_single(attacker,defender,Effect::Suffocated, chance,*suffocated_turns);
            },
            _ => {}

        }
    }
}