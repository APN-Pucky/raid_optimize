
use crate::hero::subskill::execute_subskill;
use crate::wave::InstanceIndex;
use crate::wave::Wave;
use crate::hero::effect::Effect;

use super::effect::is_dot;
use super::subskill;
use super::subskill::SubSkill;
use super::subskill::Target;
use super::subskill::merge_targets;

type SkillRef = usize;

pub struct NewSkill {
    pub cooldown : u32,
    pub subskills : Vec<SubSkill>,
}

fn false_default() -> bool{
    false
}
fn true_default() -> bool{
    true
}



#[derive(Debug, PartialEq,strum_macros::Display, Deserialize, Clone )]
pub enum Skill {
    Generic {
        cooldown: u32,
        #[serde(default="false_default")]
        basic_attack : bool,
        #[serde(rename="subskill")]
        subskills : Vec<SubSkill>,
    },
    //Liz
    ScorchedSoul {
        cooldown: u32,
        #[serde(default="true_default")]
        basic_attack : bool,
        attack_damage_ratio : f32,
        hp_burning_chance: f32,
        hp_burning_turns: u32
    },
    FireHeal {
        cooldown: u32,
        #[serde(default="false_default")]
        basic_attack : bool,
        heal_attack_ratio: f32,
        heal_max_hp_ratio: f32,
        block_debuff_turns: u32,
    },
    Resurrection {
        cooldown: u32,
        #[serde(default="false_default")]
        basic_attack : bool,
        shield_max_hp_ratio: f32,
        shield_turns: u32,
        cleanse_dot_debuffs: u32,
        restore_max_hp_ratio: f32,
    },
    // Natalie
    ScytheStrike {
        cooldown: u32,
        #[serde(default="true_default")]
        basic_attack : bool,
        attack_damage_ratio : f32,
        bleed_chance: f32,
        bleed_turns: u32
    },
    BloodthirstyScythe {
        cooldown: u32,
        #[serde(default="false_default")]
        basic_attack : bool,
        attack_damage_ratio : f32,
        bleed_chance: f32,
        bleed_turns: u32,
    },
    EnergyBurst {
        cooldown: u32,
        #[serde(default="false_default")]
        basic_attack : bool,
        attack_damage_ratio : f32,
        bleed_turns: u32,
        reduce_effect_resistance_chance :f32,
        reduce_effect_resistance_turns : u32
    },
    //Seth
    TideBigHit {
        cooldown: u32,
        #[serde(default="true_default")]
        basic_attack : bool,
        max_hp_damage_ratio : f32,
        suffocated_chance : f32,
        suffocated_turns : u32,
    },
    DeepSeaPower {
        cooldown: u32,
        #[serde(default="false_default")]
        basic_attack : bool,
        max_hp_shield_ratio : f32,
        shield_turns : u32,
        tenacity_increase_turns : u32,
    },
    CrystalOfLife {
        cooldown: u32,
        #[serde(default="false_default")]
        basic_attack : bool,
        max_hp_restore_ratio : f32,
        ripple_turns : u32,
        attack_up_turns : u32,
    },
    // Space
    Tricks { 
        cooldown: u32,
        #[serde(default="true_default")]
        basic_attack : bool,
        attack_damage_ratio : f32,
        turn_meter_reduction_ratio: f32, 
    },
    Nightmare {
        cooldown: u32,
        #[serde(default="false_default")]
        basic_attack : bool,
        attack_damage_ratio : f32,
        reduce_speed_chance : f32,
        reduce_speed_turns : u32,
        increase_speed_turns : u32,
    },
    FissionOfLife {
        cooldown: u32,
        #[serde(default="false_default")]
        basic_attack : bool,
        restore_max_hp_ratio : f32,
        heal_turns : u32,
        increase_turn_meter_ratio : f32,
    },
    // Tifya
    ScarletSlash {
        cooldown : u32,
        #[serde(default="true_default")]
        basic_attack : bool,
        attack_damage_ratio : f32,
    },
    LeavesStorm {
        cooldown : u32,
        #[serde(default="false_default")]
        basic_attack : bool,
        attack_damage_ratio : f32,
    },
    ScaletMultiStrike {
        cooldown : u32,
        #[serde(default="false_default")]
        basic_attack : bool,
        attack_damage_ratio : f32,
    },
    //
    BasicAttack { 
        cooldown: u32,
        #[serde(default="true_default")]
        basic_attack : bool,
        attack_damage_ratio : f32
    },
    DarknightStrike {
        cooldown: u32,
        #[serde(default="true_default")]
        basic_attack : bool,
        attack_damage_ratio : f32
    },
}

pub fn execute_skill<const LEN:usize>(skill : &Skill, actor :InstanceIndex, target :InstanceIndex, wave :&mut Wave<LEN>) {
    let attacker = actor;
    let defender = target;
    match skill {
        Skill::Generic{ basic_attack,cooldown, subskills ,..} => {
            for ss in subskills {
                execute_subskill(ss, actor, target, wave,skill);
            }
        },
        Skill::ScarletSlash { basic_attack, attack_damage_ratio, .. } => {
            wave.attack_single(attacker, defender,  wave.get_attack_damage(attacker)  *attack_damage_ratio, skill);
            wave.attack_single(attacker, defender,  wave.get_attack_damage(attacker)  *attack_damage_ratio, skill);
        },
        Skill::LeavesStorm { basic_attack, attack_damage_ratio, .. } => {
            wave.attack_enemy_team(attacker,   wave.get_attack_damage(attacker)  *attack_damage_ratio * (1. + 0.02 *wave.effects[actor].get(Effect::ScarletSakura).min(20) as f32) , skill);
        },
        Skill::ScaletMultiStrike{ basic_attack, attack_damage_ratio, .. } => {
            wave.attack_single(attacker, defender,  wave.get_attack_damage(attacker)  *attack_damage_ratio, skill);
            wave.attack_single(attacker, defender,  wave.get_attack_damage(attacker)  *attack_damage_ratio, skill);
            wave.attack_single(attacker, defender,  wave.get_attack_damage(attacker)  *attack_damage_ratio, skill);
            wave.attack_single(attacker, defender,  wave.get_attack_damage(attacker)  *attack_damage_ratio, skill);
            wave.attack_single(attacker, defender,  wave.get_attack_damage(attacker)  *attack_damage_ratio, skill);
        },
        Skill::Resurrection { shield_max_hp_ratio, shield_turns, cleanse_dot_debuffs, restore_max_hp_ratio ,..} => {
            let max_hp = wave.get_max_health(actor);
            wave.restore_max_hp_ratio_own_team(actor,*restore_max_hp_ratio);
            wave.shield_ally_team(actor,shield_max_hp_ratio * max_hp  ,*shield_turns);
            //TODO
            //wave.cleanse_ally_team(actor,&is_dot,*cleanse_dot_debuffs);
        },
        Skill::BloodthirstyScythe {  basic_attack,attack_damage_ratio, bleed_chance, bleed_turns ,..} =>{
            let damage = wave.get_attack_damage(actor) * attack_damage_ratio;
            wave.attack_enemy_team(actor, damage ,skill);
            wave.inflict_enemy_team(actor, Effect::Bleed,* bleed_chance,* bleed_turns);
        },
        Skill::EnergyBurst {basic_attack,  attack_damage_ratio, bleed_turns, reduce_effect_resistance_chance,  reduce_effect_resistance_turns ,..}=> {
            let damage = wave.get_attack_damage(actor) * attack_damage_ratio;
            wave.attack_enemy_team(actor, damage ,skill);
            wave.inflict_enemy_team(actor, Effect::Bleed, 1.0, *bleed_turns);
            wave.inflict_enemy_team(actor, Effect::EffectResistanceDownII, *reduce_effect_resistance_chance, *reduce_effect_resistance_turns);
        },
        Skill::DeepSeaPower {  max_hp_shield_ratio, shield_turns, tenacity_increase_turns ,..} => {
            let max_hp = wave.get_max_health(actor);
            wave.shield_ally_team(actor,max_hp_shield_ratio * max_hp  ,*shield_turns);
            wave.inflict_ally_team(actor, Effect::TenacityUpII, 1.0, *tenacity_increase_turns);
        },
        Skill::CrystalOfLife {  max_hp_restore_ratio, ripple_turns , attack_up_turns ,..} =>{
            let rest_hp = (wave.get_max_health(actor)  * max_hp_restore_ratio) ;
            wave.restore_ally_team(actor,rest_hp);
            wave.inflict_ally_team(actor, Effect::RippleII, 1.0, *ripple_turns);
            wave.inflict_ally_team(actor, Effect::AttackUpII, 1.0,* attack_up_turns);
        },
        Skill::FissionOfLife {  restore_max_hp_ratio, heal_turns, increase_turn_meter_ratio ,..} => {
            wave.restore_max_hp_ratio_own_team(actor, *restore_max_hp_ratio);
            wave.inflict_ally_team(actor, Effect::Heal, 1.0, *heal_turns);
            wave.increase_turn_meter_team(actor, *increase_turn_meter_ratio);

        }
        Skill::FireHeal{heal_attack_ratio,heal_max_hp_ratio,block_debuff_turns,..} => {
            let heal = wave.get_attack_damage(actor)*heal_attack_ratio ;
            let max_hp_heal = wave.get_max_health(actor)*heal_max_hp_ratio ;
            wave.restore(actor,target, heal + max_hp_heal);
            wave.inflict_single(actor,target,Effect::BlockDebuf, 1.0,*block_debuff_turns);
        }
        Skill::ScorchedSoul{basic_attack,attack_damage_ratio,hp_burning_chance, hp_burning_turns ,..} => {
            wave.attack_single(attacker, defender,  wave.get_attack_damage(attacker)  *attack_damage_ratio, skill);
            wave.inflict_single(attacker,defender,Effect::HPBurning, *hp_burning_chance, *hp_burning_turns);
            //wave.inflict_hp_burning(attacker,defender, *hp_burning_chance, *hp_burning_turns);
        }
        Skill::Tricks{basic_attack,attack_damage_ratio,turn_meter_reduction_ratio: turn_meter_reduction,..} => {
            wave.attack_single(attacker,defender, (wave.get_attack_damage(attacker) * attack_damage_ratio), skill);
            wave.reduce_turn_meter(attacker,defender, *turn_meter_reduction);
        }
        Skill::BasicAttack{basic_attack,attack_damage_ratio,..} => {
            wave.attack_single(attacker,defender, (wave.get_attack_damage(attacker) * attack_damage_ratio), skill);
        }
        Skill::ScytheStrike {basic_attack, attack_damage_ratio, bleed_chance,bleed_turns,.. } => {
            wave.attack_single(attacker,defender, (wave.get_attack_damage(attacker) * attack_damage_ratio), skill);
            wave.inflict_single(attacker,defender,Effect::Bleed,*bleed_chance,*bleed_turns);
        }
        Skill::DarknightStrike {basic_attack, attack_damage_ratio,.. }  => {
            wave.attack_single(attacker,defender, (wave.get_attack_damage(attacker) * attack_damage_ratio), skill);
            wave.attack_single(attacker,defender, (wave.get_attack_damage(attacker) * attack_damage_ratio), skill);
        }
        Skill::TideBigHit {basic_attack, max_hp_damage_ratio, suffocated_chance, suffocated_turns,.. } => {
            log::debug!("{} uses Tide Big Hit on {}", attacker, defender);
            let mut chance = *suffocated_chance;
            wave.attack_single(attacker,defender, (wave.get_max_health(attacker) * max_hp_damage_ratio), skill);
            if wave.has_effect(defender,Effect::WetI) 
            || wave.has_effect(defender,Effect::WetII) 
            || wave.has_effect(defender,Effect::ColdI) 
            || wave.has_effect(defender,Effect::ColdII){
                log::debug!("{} is wet or cold +15% suffocation chance", defender);
                chance += 0.15;
            }
            wave.inflict_single(attacker,defender,Effect::Suffocated, chance,*suffocated_turns);
        },
        Skill::Nightmare { basic_attack,  attack_damage_ratio, reduce_speed_chance, reduce_speed_turns, increase_speed_turns ,..} => {
            wave.attack_single(attacker,defender, (wave.get_attack_damage(attacker) * attack_damage_ratio),skill);
            wave.inflict_single(attacker,defender,Effect::SpeedDownII, *reduce_speed_chance, *reduce_speed_turns);
            wave.inflict_ally_team(actor, Effect::SpeedUpI, 1.0, *increase_speed_turns);
            //TODO target make no sense here
            //attacker.inflict(defender,Effect::SpeedUpI, 1.0, increase_speed_turns);

        }
        //_ => panic!("Skill not implemented"),
    }
    wave.cooldown_s(attacker,skill);
}

pub fn get_targets<const LEN:usize>(skill : &Skill, actor :InstanceIndex, wave :&Wave<LEN>) -> Option<Vec<InstanceIndex>> {
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
        //Space
        Skill::Tricks{..} => get_alive_enemies(actor,wave),
        Skill::Nightmare { .. } => get_alive_enemies(actor,wave),
        Skill::FissionOfLife { .. } => get_alive_allies(actor, wave),
        //Tifya
        Skill::ScarletSlash { .. } => get_alive_enemies(actor,wave),
        Skill::LeavesStorm { .. } => get_alive_enemies(actor,wave),
        Skill::ScaletMultiStrike { .. } => get_alive_enemies(actor,wave),
        //
        Skill::BasicAttack{..} => get_alive_enemies(actor,wave),
        Skill::DarknightStrike { ..} => get_alive_enemies(actor,wave),
        Skill::Generic{ subskills ,..} => Some(subskill::get_targets(get_generic_targets(subskills),actor,wave)),
    }
}

pub fn get_generic_targets(subskills : &Vec<SubSkill> ) -> Target {
    let mut targets = Vec::new();
    for ss in subskills {
        targets.push(ss.target)
    }
    let mut target = targets[0];
    for t in targets {
        target = merge_targets(target,t);
    }
    target
}

pub fn is_basic_attack(skill :&Skill) -> bool {
    match skill {
        Skill::Generic{basic_attack,..} => *basic_attack,
        Skill::ScorchedSoul{basic_attack,..} => *basic_attack,
        Skill::FireHeal{basic_attack,..} => *basic_attack,
        Skill::Resurrection{basic_attack,..} => *basic_attack,
        Skill::ScytheStrike{basic_attack,..} => *basic_attack,
        Skill::BloodthirstyScythe{basic_attack,..} => *basic_attack,
        Skill::EnergyBurst{basic_attack,..} => *basic_attack,
        Skill::TideBigHit{basic_attack,..} => *basic_attack,
        Skill::DeepSeaPower{basic_attack,..} => *basic_attack,
        Skill::CrystalOfLife{basic_attack,..} => *basic_attack,
        Skill::Tricks{basic_attack,..} => *basic_attack,
        Skill::Nightmare{basic_attack,..} => *basic_attack,
        Skill::FissionOfLife{basic_attack,..} => *basic_attack,
        Skill::ScarletSlash { cooldown, basic_attack, attack_damage_ratio } => *basic_attack,
        Skill::LeavesStorm { cooldown, basic_attack, attack_damage_ratio } => *basic_attack,
        Skill::ScaletMultiStrike { cooldown, basic_attack, attack_damage_ratio } => *basic_attack,
        Skill::BasicAttack{basic_attack,..} => *basic_attack,
        Skill::DarknightStrike{basic_attack,..} => *basic_attack,
    }
}

pub fn get_cooldown(skill: &Skill) ->u32 {
    match skill {
        //Liz
        Skill::ScorchedSoul{cooldown,..} => *cooldown,
        Skill::FireHeal { cooldown, ..} => *cooldown,
        Skill::Resurrection { cooldown, .. } => *cooldown,
        //Natalie
        Skill::ScytheStrike { cooldown,.. } => *cooldown,
        Skill::BloodthirstyScythe { cooldown,.. } => *cooldown,
        Skill::EnergyBurst { cooldown,.. } => *cooldown,
        //Seth
        Skill::TideBigHit { cooldown,..} => *cooldown,
        Skill::DeepSeaPower { cooldown, ..} => *cooldown,
        Skill::CrystalOfLife { cooldown, ..} => *cooldown,
        //Space
        Skill::Tricks{cooldown,..} => *cooldown,
        Skill::Nightmare { cooldown, ..} => *cooldown,
        Skill::FissionOfLife { cooldown, ..} => *cooldown,
        //Tifya
        Skill::ScarletSlash { cooldown, ..} => *cooldown,
        Skill::LeavesStorm { cooldown, ..} => *cooldown,
        Skill::ScaletMultiStrike { cooldown, ..} => *cooldown,
        //
        Skill::BasicAttack{cooldown,..} => *cooldown,
        Skill::DarknightStrike { cooldown,..} => *cooldown,
        Skill::Generic{ cooldown, ..} => *cooldown,
    }
}

// TODO why option and not just empty array?!?!?

fn get_alive_allies<const LEN:usize>(actor :  InstanceIndex, wave : &Wave<LEN>) -> Option<Vec<InstanceIndex>> {
    let team = wave.get_ally_indices(actor);
    let mut ids = Vec::new();
    for (index,&target) in team.iter().enumerate() {
        if wave.is_alive(target)  {
            ids.push(target);
        }
    }
    if ids.is_empty() {
        None
    }
    else {
        Some(ids)
    }
}

fn get_alive_enemies<const LEN:usize>(actor :InstanceIndex, wave :&Wave<LEN>) -> Option<Vec<InstanceIndex>> {
    let team = wave.get_enemies_indices(actor);
    let mut ids = Vec::new();
    for (index,&target) in team.iter().enumerate() {
        if wave.is_alive(target)  {
            ids.push(target);
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