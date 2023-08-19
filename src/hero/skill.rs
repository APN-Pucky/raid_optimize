use crate::wave::Wave;
use crate::wave::InstanceRef;
use crate::hero::instance::Instance;
use crate::hero::effect::Effect;

#[derive(Debug, PartialEq,strum_macros::Display, Deserialize, Clone ,Copy)]
pub enum Skill {
    ScorchedSoul {
        cooldown: u32,
        attack_damage_ratio : f32,
        hp_burning_chance: f32,
        hp_burning_turns: u32
    },
    Tricks { 
        cooldown: u32,
        attack_damage_ratio : f32,
        turn_meter_reduction: f32, 
    },
    BasicAttack { 
        cooldown: u32,
        attack_damage_ratio : f32
    },
    ScytheStrike {
        cooldown: u32,
        attack_damage_ratio : f32,
        bleed_chance: f32,
        bleed_turns: u32
    },
    DarknightStrike {
        cooldown: u32,
        attack_damage_ratio : f32
    },
    TideBigHit {
        cooldown: u32,
        max_hp_damage_ratio : f32,
        suffocated_chance : f32,
        suffocated_turns : u32,
    }
}

pub fn execute_skill(skill : Skill, actor :&InstanceRef, target :&InstanceRef, wave :&mut Wave) {
    let mut attacker;
    let mut defender;
    if actor.team {
        attacker = &mut wave.allies[actor.index];
        defender = &mut wave.enemies[target.index];
    }
    else {
        attacker = &mut wave.enemies[actor.index];
        defender = &mut wave.allies[target.index];
    }        
    match skill {
        Skill::ScorchedSoul{attack_damage_ratio,hp_burning_chance, hp_burning_turns ,..} => {
            attacker.attack(defender,  (attacker.get_attack_damage() as f32 * attack_damage_ratio) as u32);
            attacker.inflict(defender, Effect::HPBurning, hp_burning_chance, hp_burning_turns);
        }
        Skill::Tricks{attack_damage_ratio,turn_meter_reduction,..} => {
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
        }
    }
    // cooldown the used skill
    attacker.cooldown(skill);
}

pub fn get_targets(skill : Skill, actor :&InstanceRef, wave :&Wave) -> Option<Vec<InstanceRef>> {
    match skill {
        Skill::ScorchedSoul{..} => get_alive_enemies(actor,wave),
        Skill::Tricks{..} => get_alive_enemies(actor,wave),
        Skill::BasicAttack{..} => get_alive_enemies(actor,wave),
        Skill::ScytheStrike { .. } => get_alive_enemies(actor,wave),
        Skill::TideBigHit { ..} => get_alive_enemies(actor,wave),
        Skill::DarknightStrike { ..} => get_alive_enemies(actor,wave),
    }
}

pub fn get_cooldown(skill: Skill) ->u32 {
    match skill {
        Skill::ScorchedSoul{cooldown,..} => cooldown,
        Skill::Tricks{cooldown,..} => cooldown,
        Skill::BasicAttack{cooldown,..} => cooldown,
        Skill::ScytheStrike { cooldown,.. } => cooldown,
        Skill::TideBigHit { cooldown,..} => cooldown,
        Skill::DarknightStrike { cooldown,..} => cooldown,
    }
}

fn get_alive_enemies(actor :&InstanceRef, wave :&Wave) -> Option<Vec<InstanceRef>> {
    let team = wave.get_enemy_team(actor);
    let mut ids = Vec::new();
    let mut index = 0;
    for target in team.iter() {
        if target.is_alive()  {
            ids.push(index);
        }
        index += 1;
    }
    if ids.len() == 0 {
        return None;
    }
    else {
        let mut targets = Vec::new();
        for i in 0..ids.len() {
            targets.push(InstanceRef{team: !actor.team, index: ids[i]});
        }
        return Some(targets);
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