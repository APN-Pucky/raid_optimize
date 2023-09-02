use crate::{wave::{Wave, InstanceIndex}, data::{skill::Skill, effect::Effect}};


impl<'a,const LEN:usize> Wave<'a,LEN> {
    pub fn execute_skill_natalie(&mut self,  skill : &Skill, actor :InstanceIndex, target :InstanceIndex, ) {
        let attacker = actor;
        let defender = target;
        match skill {
            Skill::BloodthirstyScythe {  basic_attack,attack_damage_ratio, bleed_chance, bleed_turns ,..} =>{
                let damage = self.get_attack_damage(actor) * attack_damage_ratio;
                self.attack_enemy_team(actor, damage ,skill);
                self.inflict_enemy_team(actor, Effect::Bleed,* bleed_chance,* bleed_turns);
            },
            Skill::EnergyBurst {basic_attack,  attack_damage_ratio, bleed_turns, reduce_effect_resistance_chance,  reduce_effect_resistance_turns ,..}=> {
                let damage = self.get_attack_damage(actor) * attack_damage_ratio;
                self.attack_enemy_team(actor, damage ,skill);
                self.inflict_enemy_team(actor, Effect::Bleed, 1.0, *bleed_turns);
                self.inflict_enemy_team(actor, Effect::EffectResistanceDownII, *reduce_effect_resistance_chance, *reduce_effect_resistance_turns);
            },
            Skill::ScytheStrike {basic_attack, attack_damage_ratio, bleed_chance,bleed_turns,.. } => {
                self.attack_single(attacker,defender, (self.get_attack_damage(attacker) * attack_damage_ratio), skill);
                self.inflict_single(attacker,defender,Effect::Bleed,*bleed_chance,*bleed_turns);
            }
            _ => {}

        }
    }
}