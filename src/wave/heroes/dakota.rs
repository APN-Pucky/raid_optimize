use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill, SkillData}, effect::Effect}, debug};


impl<'a,const LEN:usize> Wave<'a,LEN> {
    pub fn execute_skill_dakota(&mut self,  skill : &Skill, actor :InstanceIndex, target :InstanceIndex, ) {
        let attacker = actor;
        let defender = target;
        match skill.data {
            SkillData::SoulSurge {toxic_swamp_turns, rose_poison_chance, speed_up_turns } => {
                self.inflict_enemy_team(actor, Effect::ToxicSwamp, 1.0, toxic_swamp_turns);
                self.inflict_ally_team(actor, Effect::SpeedUpII, 1.0, speed_up_turns);
            },
            SkillData::SoulSeal{ rose_per_poison, attack_damage_ratio, poison_turns ,attack_damage_ratio_per_poison, increase_atk_turns } => {
                self.inflict_ally_team(actor, Effect::AttackUpII, 1.0, increase_atk_turns);
                // counter number of effects arcane
                let mut poison = self.effects[target].get(Effect::Poison);
                let mut rose_poison = self.effects[target].get(Effect::RosePoison);

                self.effects[target].clear_single(Effect::Poison);
                self.attack_single(actor, target, self.get_attack_damage(actor) * (attack_damage_ratio + poison as f32 * attack_damage_ratio_per_poison), skill);

                while self.effects[target].get(Effect::RosePoison) >= rose_per_poison {
                    self.inflict_single(actor, target, Effect::Poison, 1.0, poison_turns );
                    self.effects[target].remove_layers(Effect::RosePoison,rose_per_poison);
                }

            },
            _ => {}

        }
    }

    pub fn on_attacked_dakota(&mut self, attacker : InstanceIndex, attacked:InstanceIndex) {
        match self.heroes[attacked].skills[..] {
            [Skill {data : SkillData::SoulRing{effect_res_down_chance, effect_res_down_turns },..},..] => {
                self.inflict_single(attacked, attacker, Effect::RosePoison, 1.0, 999);
                self.inflict_single(attacked, attacker, Effect::EffectResistanceDownII, effect_res_down_chance,effect_res_down_turns);
            },
            _ => {}
        }
    }

    pub fn on_inflict_dakota(&mut self, actor : InstanceIndex, target:InstanceIndex, effect : Effect, turns :&mut u32) {
        if self.has_effect(target,Effect::ToxicSwamp) && effect == Effect::Poison {
            let inflictor = self.effects[target].get_last_inflictor(Effect::ToxicSwamp);
            if self.is_alive(inflictor) {
                debug!("{} has ToxicSwamp, Poison prolonged by one turn", self.name(target));
                // increase turns by one
                *turns = *turns+1;
                match self.heroes[inflictor].skills[..] {
                    [Skill {data: SkillData::SoulSurge {rose_poison_chance,..} , .. },..] => {
                        self.inflict_single(inflictor, target, Effect::RosePoison, rose_poison_chance, 999)
                    }
                    _ => {}
                }
            }
        }
    }
}