
use crate::wave::Wave;
use crate::wave::InstanceRef;
use crate::hero::effect::Effect;

use super::effect::is_dot;

#[derive(Debug, PartialEq,strum_macros::Display, Deserialize, Clone ,Copy)]
pub enum Skill {
    //Liz
    ScorchedSoul {
        cooldown: u32,
        attack_damage_ratio : f32,
        hp_burning_chance: f32,
        hp_burning_turns: u32
    },
    FireHeal {
        cooldown: u32,
        heal_attack_ratio: f32,
        heal_max_hp_ratio: f32,
        block_debuff_turns: u32,
    },
    Resurrection {
        cooldown: u32,
        shield_max_hp_ratio: f32,
        shield_turns: u32,
        cleanse_dot_debuffs: u32,
        restore_max_hp_ratio: f32,
    },
    // Natalie
    ScytheStrike {
        cooldown: u32,
        attack_damage_ratio : f32,
        bleed_chance: f32,
        bleed_turns: u32
    },
    BloodthirstyScythe {
        cooldown: u32,
        attack_damage_ratio : f32,
        bleed_chance: f32,
        bleed_turns: u32,
    },
    EnergyBurst {
        cooldown: u32,
        attack_damage_ratio : f32,
        bleed_turns: u32,
        reduce_effect_resistance_chance :f32,
        reduce_effect_resistance_turns : u32
    },
    //Seth
    TideBigHit {
        cooldown: u32,
        max_hp_damage_ratio : f32,
        suffocated_chance : f32,
        suffocated_turns : u32,
    },
    DeepSeaPower {
        cooldown: u32,
        max_hp_shield_ratio : f32,
        shield_turns : u32,
        tenacity_increase_turns : u32,
    },
    CrystalOfLife {
        cooldown: u32,
        max_hp_restore_ratio : f32,
        ripple_turns : u32,
        attack_up_turns : u32,
    },
    // Space
    Tricks { 
        cooldown: u32,
        attack_damage_ratio : f32,
        turn_meter_reduction_ratio: f32, 
    },
    Nightmare {
        cooldown: u32,
        attack_damage_ratio : f32,
        reduce_speed_chance : f32,
        reduce_speed_turns : u32,
        increase_speed_turns : u32,
    },
    FissionOfLife {
        cooldown: u32,
        restore_max_hp_ratio : f32,
        heal_turns : u32,
        increase_turn_meter_ratio : f32,
    },
    //
    BasicAttack { 
        cooldown: u32,
        attack_damage_ratio : f32
    },
    DarknightStrike {
        cooldown: u32,
        attack_damage_ratio : f32
    },


}
pub fn execute_skill(skill : Skill, actor :&InstanceRef, target :&InstanceRef, wave :&mut Wave) {
    match skill {
        Skill::Resurrection { shield_max_hp_ratio, shield_turns, cleanse_dot_debuffs, restore_max_hp_ratio ,..} => {
            let max_hp = wave.get_instance(*actor).get_max_health();
            wave.restore_max_hp_ratio_own_team(actor,restore_max_hp_ratio);
            wave.shield_team(actor,(shield_max_hp_ratio * max_hp as f32) as u32,shield_turns);
            wave.cleanse_team(actor,&is_dot,cleanse_dot_debuffs);
        },
        Skill::BloodthirstyScythe {  attack_damage_ratio, bleed_chance, bleed_turns ,..} =>{
            let damage = wave.get_instance(*actor).get_attack_damage() as f32 * attack_damage_ratio;
            wave.attack_team(actor, damage as u32);
            wave.inflict_team(actor, Effect::Bleed, bleed_chance, bleed_turns);
        },
        Skill::EnergyBurst {  attack_damage_ratio, bleed_turns, reduce_effect_resistance_chance,  reduce_effect_resistance_turns ,..}=> {
            let damage = wave.get_instance(*actor).get_attack_damage() as f32 * attack_damage_ratio;
            wave.attack_team(actor, damage as u32);
            wave.inflict_team(actor, Effect::Bleed, 1.0, bleed_turns);
            wave.inflict_team(actor, Effect::EffectResistanceDownII, reduce_effect_resistance_chance, reduce_effect_resistance_turns);
        },
        Skill::DeepSeaPower {  max_hp_shield_ratio, shield_turns, tenacity_increase_turns ,..} => {
            let max_hp = wave.get_instance(*actor).get_max_health();
            wave.shield_team(actor,(max_hp_shield_ratio * max_hp as f32) as u32,shield_turns);
            wave.inflict_team(actor, Effect::TenacityUpII, 1.0, tenacity_increase_turns);
        },
        Skill::CrystalOfLife {  max_hp_restore_ratio, ripple_turns , attack_up_turns ,..} =>{
            let rest_hp = (wave.get_instance(*actor).get_max_health() as f32 * max_hp_restore_ratio) as u32;
            wave.restore_max_hp_own_team(actor,rest_hp);
            wave.inflict_team(actor, Effect::RippleII, 1.0, ripple_turns);
            wave.inflict_team(actor, Effect::AttackUpII, 1.0, attack_up_turns);
        },
        Skill::FissionOfLife {  restore_max_hp_ratio, heal_turns, increase_turn_meter_ratio ,..} => {
            wave.restore_max_hp_ratio_own_team(actor, restore_max_hp_ratio);
            wave.inflict_team(actor, Effect::Heal, 1.0, heal_turns);
            wave.increase_turn_meter_team(actor, increase_turn_meter_ratio);

        }
        _ => execute_skill_1_on_1(skill, actor, target, wave),
    }
}

pub fn execute_skill_1_on_1(skill : Skill, actor :&InstanceRef, target :&InstanceRef, wave :&mut Wave) {
    let attacker;
    let defender;
    if actor.team {
        attacker = &mut wave.allies[actor.index];
        defender = &mut wave.enemies[target.index];
    }
    else {
        attacker = &mut wave.enemies[actor.index];
        defender = &mut wave.allies[target.index];
    }        
    match skill {
        Skill::FireHeal{heal_attack_ratio,heal_max_hp_ratio,block_debuff_turns,..} => {
            let heal = (attacker.get_attack_damage() as f32 * heal_attack_ratio) as u32;
            let max_hp_heal = (defender.get_max_health() as f32 * heal_max_hp_ratio) as u32;
            attacker.restore(defender, heal + max_hp_heal);
            attacker.inflict(defender,Effect::BlockDebuf, 1.0,block_debuff_turns);
        }
        Skill::ScorchedSoul{attack_damage_ratio,hp_burning_chance, hp_burning_turns ,..} => {
            attacker.attack(defender,  (attacker.get_attack_damage() as f32 * attack_damage_ratio) as u32);
            attacker.inflict_hp_burning(defender, hp_burning_chance, hp_burning_turns);
        }
        Skill::Tricks{attack_damage_ratio,turn_meter_reduction_ratio: turn_meter_reduction,..} => {
            attacker.attack(defender, (attacker.get_attack_damage() as f32 * attack_damage_ratio) as u32);
            attacker.reduce_turn_meter(defender, turn_meter_reduction);
        }
        Skill::BasicAttack{attack_damage_ratio,..} => {
            attacker.attack(defender, (attacker.get_attack_damage() as f32 * attack_damage_ratio) as u32);
        }
        Skill::ScytheStrike { attack_damage_ratio, bleed_chance,bleed_turns,.. } => {
            attacker.attack(defender, (attacker.get_attack_damage() as f32 * attack_damage_ratio) as u32);
            attacker.inflict_bleed(defender,bleed_chance,bleed_turns);
        }
        Skill::DarknightStrike { attack_damage_ratio,.. }  => {
            attacker.attack(defender, (attacker.get_attack_damage() as f32 * attack_damage_ratio) as u32);
            attacker.attack(defender, (attacker.get_attack_damage() as f32 * attack_damage_ratio) as u32);
        }
        Skill::TideBigHit { max_hp_damage_ratio, suffocated_chance, suffocated_turns,.. } => {
            log::debug!("{} uses Tide Big Hit on {}", attacker, defender);
            let mut chance = suffocated_chance;
            attacker.attack(defender, (attacker.get_max_health() as f32 * max_hp_damage_ratio) as u32);
            if defender.has_effect(Effect::WetI) 
            || defender.has_effect(Effect::WetII) 
            || defender.has_effect(Effect::ColdI) 
            || defender.has_effect(Effect::ColdII){
                log::debug!("{} is wet or cold +15% suffocation chance", defender);
                chance += 0.15;
            }
            attacker.inflict(defender,Effect::Suffocated, chance,suffocated_turns);
        },
        Skill::Nightmare {  attack_damage_ratio, reduce_speed_chance, reduce_speed_turns, increase_speed_turns ,..} => {
            attacker.attack(defender, (attacker.get_attack_damage() as f32 * attack_damage_ratio) as u32);
            attacker.inflict(defender,Effect::SpeedDownII, reduce_speed_chance, reduce_speed_turns);
            attacker.inflict(defender,Effect::SpeedUpI, 1.0, increase_speed_turns);

        }
        _ => panic!("Skill not implemented"),
    }
    // cooldown the used skill
    attacker.cooldown(skill);
}

pub fn get_targets(skill : Skill, actor :&InstanceRef, wave :&Wave) -> Option<Vec<InstanceRef>> {
    match skill {
        //Liz
        Skill::ScorchedSoul{..} => get_alive_enemies(actor,wave),
        Skill::FireHeal{..} => get_alive_allies(actor,wave),
        Skill::Resurrection { .. } => get_alive_allies(actor,wave),
        //Natalie
        Skill::ScytheStrike { .. } => get_alive_enemies(actor,wave),
        Skill::BloodthirstyScythe { .. } => get_alive_enemies(actor,wave),
        Skill::EnergyBurst { .. } => get_alive_enemies(actor,wave),
        //Seth
        Skill::TideBigHit { ..} => get_alive_enemies(actor,wave),
        Skill::DeepSeaPower { .. } => get_alive_allies(actor, wave),
        Skill::CrystalOfLife { .. } => get_alive_allies(actor, wave),
        //Sapce
        Skill::Tricks{..} => get_alive_enemies(actor,wave),
        Skill::Nightmare { .. } => get_alive_enemies(actor,wave),
        Skill::FissionOfLife { .. } => get_alive_allies(actor, wave),
        //
        Skill::BasicAttack{..} => get_alive_enemies(actor,wave),
        Skill::DarknightStrike { ..} => get_alive_enemies(actor,wave),
    }
}

pub fn get_cooldown(skill: Skill) ->u32 {
    match skill {
        //Liz
        Skill::ScorchedSoul{cooldown,..} => cooldown,
        Skill::FireHeal { cooldown, ..} => cooldown,
        Skill::Resurrection { cooldown, .. } => cooldown,
        //Natalie
        Skill::ScytheStrike { cooldown,.. } => cooldown,
        Skill::BloodthirstyScythe { cooldown,.. } => cooldown,
        Skill::EnergyBurst { cooldown,.. } => cooldown,
        //Seth
        Skill::TideBigHit { cooldown,..} => cooldown,
        Skill::DeepSeaPower { cooldown, ..} => cooldown,
        Skill::CrystalOfLife { cooldown, ..} => cooldown,
        //Space
        Skill::Tricks{cooldown,..} => cooldown,
        Skill::Nightmare { cooldown, ..} => cooldown,
        Skill::FissionOfLife { cooldown, ..} => cooldown,
        Skill::BasicAttack{cooldown,..} => cooldown,
        Skill::DarknightStrike { cooldown,..} => cooldown,
    }
}

fn get_alive_allies(actor : & InstanceRef, wave : &Wave) -> Option<Vec<InstanceRef>> {
    let team = wave.get_ally_team(actor);
    let mut ids = Vec::new();
    for (index,target) in team.iter().enumerate() {
        if target.is_alive()  {
            ids.push(InstanceRef{team: actor.team, index});
        }
    }
    if ids.is_empty() {
        None
    }
    else {
        Some(ids)
    }
}

fn get_alive_enemies(actor :&InstanceRef, wave :&Wave) -> Option<Vec<InstanceRef>> {
    let team = wave.get_enemy_team(actor);
    let mut ids = Vec::new();
    for (index,target) in team.iter().enumerate() {
        if target.is_alive()  {
            ids.push(InstanceRef{team: !actor.team, index});
        }
    }
    if ids.is_empty() {
        None
    }
    else {
        Some(ids)
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn read_xml() {
        let skill: Skill= serde_xml_rs::from_str(
            r#"
            <ScorchedSoul>
                <cooldown>3</cooldown>
                <attack_damage_ratio>1.0</attack_damage_ratio>
                <hp_burning_chance>0.5</hp_burning_chance>
                <hp_burning_turns>2</hp_burning_turns>
            </ScorchedSoul>
            "#,
        )
        .unwrap();

        match skill {
            Skill::ScorchedSoul{attack_damage_ratio,hp_burning_chance, hp_burning_turns ,..} => {
                assert_eq!(attack_damage_ratio, 1.0);
                assert_eq!(hp_burning_chance, 0.5);
                assert_eq!(hp_burning_turns, 2);
            }
            _ => panic!("Wrong skill type"),
        }
    }
}