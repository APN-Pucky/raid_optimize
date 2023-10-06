use std::fmt;
use strum_macros::EnumIter;

use crate::wave::heroes::alahan::detach::Detach;

use crate::wave::heroes::marville::fish_dive::FishDive;
use crate::wave::heroes::marville::fish_guardian::FishGuardian;
use crate::wave::heroes::marville::fish_waterball::FishWaterball;
use crate::wave::heroes::marville::clean_ocean::CleanOcean;
use crate::wave::heroes::natalie::bloodthristy_desire::BloodthirstyDesire;
use crate::wave::heroes::hazier::bloodlust_strike::BloodlustStrike;
use crate::wave::heroes::maya::force_of_mercy::ForceOfMercy;
use crate::wave::heroes::maya::light_of_purifying::LightOfPurifying;
use crate::wave::heroes::maya::sacred_light::SacredLight;
use crate::wave::heroes::BasicAttack;
use crate::wave::heroes::dakota::soul_ring::SoulRing;
use crate::wave::heroes::dakota::soul_seal::SoulSeal;
use crate::wave::heroes::dakota::soul_surge::SoulSurge;
use crate::wave::heroes::margarita::counterattack_command::CounterattackCommand;
use crate::wave::heroes::PassiveSkill;
use crate::wave::InstanceIndex;
use crate::wave::Wave;
use crate::wave::heroes::alahan::commendation::Commendation;
use crate::wave::heroes::alahan::spirit_call::SpiritCall;
use crate::wave::heroes::alahan::spirit_fountain::SpiritFountain;
use crate::wave::heroes::geeliman::bursting_knowledge::BurstingKnowledge;
use crate::wave::heroes::hazier::darknight_arbitrament::DarknightArbitrament;
use crate::wave::heroes::hazier::darknight_strike::DarknightStrike;
use crate::wave::heroes::hazier::eye_for_an_eye::EyeForAnEye;
use crate::wave::heroes::liz::fire_heal::FireHeal;
use crate::wave::heroes::liz::resurrection::Resurrection;
use crate::wave::heroes::liz::scorched_soul::ScorchedSoul;
use crate::wave::heroes::natalie::bloodthirsty_scythe::BloodthirstyScythe;
use crate::wave::heroes::natalie::energy_burst::EnergyBurst;
use crate::wave::heroes::natalie::scythe_strike::ScytheStrike;
use crate::wave::heroes::seth::crystal_of_life::CrystalOfLife;
use crate::wave::heroes::seth::deep_sea_power::DeepSeaPower;
use crate::wave::heroes::seth::tide_big_hit::TideBigHit;
use crate::wave::heroes::space::fission_of_life::FissionOfLife;
use crate::wave::heroes::space::nightmare::Nightmare;
use crate::wave::heroes::space::resplendence::Resplendence;
use crate::wave::heroes::space::tricks::Tricks;
use crate::wave::heroes::tifya::leaves_storm::LeavesStorm;
use crate::wave::heroes::tifya::scarlet_multi_strike::ScarletMultiStrike;
use crate::wave::heroes::tifya::scarlet_slash::ScarletSlash;
use crate::wave::heroes::Cooldown;
use crate::wave::heroes::Execute;
use crate::wave::heroes::Typed;
use crate::wave::heroes::Selector;


use super::subskill::SubSkill;




type SkillRef = usize;
use strum_macros::EnumString;

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

#[derive(Debug, PartialEq,strum_macros::Display, Deserialize, Serialize, Clone,Copy )]
pub enum SkillType{
    Basic,
    Passive,
    Active,
    None,
}

pub fn cooldown_default() -> u32 {
    0
}

pub fn select_default() -> Select {
    Select::None
}

pub fn typ_default() -> SkillType {
    SkillType::None
}

#[derive(Deserialize, Serialize,strum_macros::Display, Debug, Clone,Eq, PartialEq,Copy)]
pub enum Select {
    Everyone,
    SingleAlly,
    SingleEnemy,
    AllEnemies,
    AllAllies,
    SingleSelf,
    None,
}

impl Default for Select {
    fn default() -> Self {
        Select::None
    }
}


//#[derive(Debug, PartialEq, Deserialize, Serialize, Clone )]
//pub struct Skill {
//    //#[serde(default="cooldown_default")]
//    //pub cooldown : u32,
//    //#[serde(default="typ_default", rename="type",with = "quick_xml::serde_helpers::text_content")]
//    //pub typ : SkillType,
//    //#[serde(default="select_default",with = "quick_xml::serde_helpers::text_content")]
//    //pub select: Select,
//    #[serde(rename = "$value")]
//    pub data : SkillData,
//}

//impl fmt::Display for Skill{
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        write!(f, "{}", self.data)
//    }
//}

//pub const NONE_SKILL: Skill = Skill {
//    //cooldown : 0,
//    //typ : SkillType::None,
//    //select : Select::None,
//    data : SkillData::None,
//};
//
//pub const BASIC_ATTACK: Skill = Skill {
//    //cooldown : 0,
//    //typ : SkillType::Basic,
//    //select : Select::SingleEnemy,
//    data : SkillData::BasicAttack (
//        BasicAttack{
//            cooldown : 0,
//            attack_damage_ratio : 1.0,
//        }
//    ),
//};


//#[derive(Default,Debug, PartialEq, Deserialize, Serialize, Clone )]
//pub struct ScorchedSoul{
//    attack_damage_ratio : f32,
//    hp_burning_chance: f32,
//    hp_burning_turns: u32
//}


//#[derive(EnumString, EnumIter, Debug, PartialEq,strum_macros::Display, Deserialize, Serialize, Clone )]
//pub enum SkillData {
//    None,
//    // Stabilized
//    // Tested
//    // Prototyped
//
//    BasicAttack(BasicAttack), 
//    //BasicAttack {
//    //    attack_damage_ratio : f32,
//    //},
//    //Liz
//    ScorchedSoul(ScorchedSoul),
//    //ScorchedSoul {
//    //    attack_damage_ratio : f32,
//    //    hp_burning_chance: f32,
//    //    hp_burning_turns: u32
//    //},
//    FireHeal (FireHeal),
//    Resurrection (Resurrection),
//    // Natalie
//    ScytheStrike(ScytheStrike),
//    BloodthirstyScythe(BloodthirstyScythe) ,
//    EnergyBurst(EnergyBurst) ,
//    //Seth
//    TideBigHit(TideBigHit),
//    DeepSeaPower(DeepSeaPower),
//    CrystalOfLife(CrystalOfLife),
//    // Space
//    Tricks(Tricks),
//    Nightmare(Nightmare),
//    Resplendence(Resplendence) ,
//    FissionOfLife(FissionOfLife),
//    // Tifya
//    ScarletSlash(ScarletSlash),
//    LeavesStorm(LeavesStorm),
//    ScaletMultiStrike(ScarletMultiStrike),
//    //Hazier
//    DarknightStrike(DarknightStrike) ,
//    EyeForAnEye (EyeForAnEye),
//    DarknightArbitrament(DarknightArbitrament) ,
//    //Geeliman
//    BurstingKnowledge(BurstingKnowledge) ,
//    //Alahan
//    SpiritCall {
//        attack_damage_ratio: f32,
//        restore_hp_damage_ratio: f32,
//        remove_all_buffs: bool,
//        heal_lowest_ally : bool,
//        increase_hp : bool,
//    },
//    SpiritFountain {
//        heal_turns: u32,
//        cleanse_attribute_debuffs: bool,
//    },
//    Commendation {
//        max_hp_restore_ratio: f32,
//        attack_up_turns : u32,
//    },
//    Detach {
//        attack_damage_ratio : f32,
//        stun_chance: f32,
//        stun_turns: u32,
//        steal_shield: bool,
//        shield_max_hp_ratio: f32,
//        shield_turns: u32,
//    },
//    //Marville
//    FishWaterball {
//        attack_damage_ratio : f32,
//        act_chance: f32,
//    },
//    CleanOcean {
//        restore_max_hp_ratio : f32,
//        cleanse_dot_layers : u32,
//        consolidation_turns : u32,
//        block_removal_turns  : u32,
//    },
//    FishGuardian {
//        restore_fish_shoal: u32,
//        max_hp_restore_ratio : f32,
//        damage_reduction : f32
//    },
//    FishDive {
//        restore_fish_shoal: u32,
//    },
//    //Dakota
//    SoulSurge {
//        toxic_swamp_turns : u32,
//        rose_poison_chance : f32,
//        speed_up_turns : u32,
//    },
//    SoulRing {
//        effect_res_down_chance : f32,
//        effect_res_down_turns : u32,
//    },
//    SoulSeal {
//        attack_damage_ratio : f32,
//        attack_damage_ratio_per_poison : f32,
//        increase_atk_turns : u32,
//        rose_per_poison : u32,
//        poison_turns : u32
//    },
//    // Maya
//    LightOfPurifying {
//        heal_allies : u32,
//        max_hp_restore_ratio : f32,
//        heal_turns : u32,
//        cleanse_dot_layers: u32,
//    },
//    ForceOfMercy {
//        max_hp_restore_ratio : f32,
//        healing_effect : f32,
//    },
//    SacredLight {
//        max_hp_restore_ratio : f32,
//        loose_hp_ratio : f32,
//        consolidation_turns : u32,
//        shield_turns : u32,
//        shield_max_hp_ratio : f32,
//        block_debuff_turns : u32,
//    },
//
//
//    //Natalie
//    BloodthirstyDesire,
//    //Seth
//    DeepSeaBloodline,
//    //Space
//
//    //Tifya
//    SharpInstinct,
//    //Hazier
//    BloodlustStrike {
//        leech : f32,
//        damage_reduction_buffs : f32,
//        damage_reduction_nobuffs : f32,
//    },
//    IncessantChatter, // TODO
//    //Margarita
//    CounterattackCommand {
//        blades : u32,
//        crit_damage_turns : u32,
//        attack_damage_ratio : f32,
//    },
//}

pub fn get_selection(wave :& Wave, select: Select, actor :InstanceIndex, ) -> Vec<InstanceIndex> {
    match select{
        Select::Everyone => {
            // 0..LEN
            wave.get_indices()
        },
        Select::SingleAlly => {
            wave.get_ally_indices(actor)
        },
        Select::SingleEnemy => {
            wave.get_enemies_indices(actor)
        },
        Select::AllEnemies => {
            wave.get_enemies_indices(actor)
        },
        Select::AllAllies => {
            wave.get_ally_indices(actor)
        },
        Select::SingleSelf => {
            vec![actor]
        },
        Select::None => {
            vec![]
        }
    } 
}



//pub fn get_selection<const LEN:usize>(skill : &Skill, actor :InstanceIndex, wave :&Wave<LEN>) -> Vec<InstanceIndex> {
//    return get_subskill_targets(skill.select, actor, wave);
//    //match skill {
//    //    //Liz
//    //    Skill::ScorchedSoul{..} => get_alive_enemies(actor,wave),
//    //    Skill::FireHeal{..} => get_alive_allies(actor,wave),
//    //    Skill::Resurrection { .. } => get_alive_allies(actor,wave),
//    //    //Natalie
//    //    Skill::ScytheStrike { .. } => get_alive_enemies(actor,wave),
//    //    Skill::BloodthirstyScythe { .. } => get_alive_enemies(actor,wave),
//    //    Skill::EnergyBurst { .. } => get_alive_enemies(actor,wave),
//    //    //Seth
//    //    Skill::TideBigHit { ..} => get_alive_enemies(actor,wave),
//    //    Skill::DeepSeaPower { .. } => get_alive_allies(actor, wave),
//    //    Skill::CrystalOfLife { .. } => get_alive_allies(actor, wave),
//    //    //Space
//    //    Skill::Tricks{..} => get_alive_enemies(actor,wave),
//    //    Skill::Nightmare { .. } => get_alive_enemies(actor,wave),
//    //    Skill::FissionOfLife { .. } => get_alive_allies(actor, wave),
//    //    //Tifya
//    //    Skill::ScarletSlash { .. } => get_alive_enemies(actor,wave),
//    //    Skill::LeavesStorm { .. } => get_alive_enemies(actor,wave),
//    //    Skill::ScaletMultiStrike { .. } => get_alive_enemies(actor,wave),
//    //    //Hazier
//    //    Skill::DarknightStrike { ..} => get_alive_enemies(actor,wave),
//    //    Skill::EyeForAnEye { .. } => Some(vec![actor]),
//    //    Skill::DarknightArbitrament { .. } => get_alive_enemies(actor, wave),
//    //    //Geeliman
//    //    Skill::BurstingKnowledge { .. } => get_alive_enemies(actor, wave),
//    //    //
//    //    Skill::BasicAttack{..} => get_alive_enemies(actor,wave),
//    //    Skill::Generic{ subskills ,..} => Some(subskill::get_subskill_targets(get_generic_targets(subskills),actor,wave)),
//    //}
//}

pub fn is_passive(skill : &Skill) -> bool {
    return get_type(skill) == SkillType::Passive;
}

pub fn is_reducable(skill : &Skill) -> bool {
    match skill {
        Skill::FishDive {..} => false,
        _ => true,
    }
}

pub fn is_basic_attack(skill :&Skill) -> bool {
    return get_type(skill) == SkillType::Basic;
    //match skill {
    //    Skill::Generic{basic_attack,..} => *basic_attack,
    //    Skill::ScorchedSoul{basic_attack,..} => *basic_attack,
    //    Skill::FireHeal{basic_attack,..} => *basic_attack,
    //    Skill::Resurrection{basic_attack,..} => *basic_attack,
    //    Skill::ScytheStrike{basic_attack,..} => *basic_attack,
    //    Skill::BloodthirstyScythe{basic_attack,..} => *basic_attack,
    //    Skill::EnergyBurst{basic_attack,..} => *basic_attack,
    //    Skill::TideBigHit{basic_attack,..} => *basic_attack,
    //    Skill::DeepSeaPower{basic_attack,..} => *basic_attack,
    //    Skill::CrystalOfLife{basic_attack,..} => *basic_attack,
    //    Skill::Tricks{basic_attack,..} => *basic_attack,
    //    Skill::Nightmare{basic_attack,..} => *basic_attack,
    //    Skill::FissionOfLife{basic_attack,..} => *basic_attack,
    //    Skill::ScarletSlash { cooldown, basic_attack, attack_damage_ratio } => *basic_attack,
    //    Skill::LeavesStorm { cooldown, basic_attack, attack_damage_ratio } => *basic_attack,
    //    Skill::ScaletMultiStrike { cooldown, basic_attack, attack_damage_ratio } => *basic_attack,
    //    Skill::BasicAttack{basic_attack,..} => *basic_attack,
    //    Skill::DarknightStrike{basic_attack,..} => *basic_attack,
    //    Skill::EyeForAnEye{basic_attack,..} => *basic_attack,
    //    Skill::DarknightArbitrament{basic_attack,..} => *basic_attack,
    //    Skill::BurstingKnowledge{basic_attack,..} => *basic_attack,
    //}
}

macro_rules! gen_match {
    ( [$($Passive:ident),*],
    //[$($Passive_extra:ident {$($Passive_extra1:ident : $Passive_extra2:ident),*}),*],
      [$($Special:ident),*] ) => {
        #[derive(EnumString, EnumIter, Debug, PartialEq,strum_macros::Display, Deserialize, Serialize, Clone )]
        pub enum Skill {
            $($Special ($Special),)*
            $($Passive,)*
            None,
            Generic {
                cooldown : u32,
                name : String,
                #[serde(default="select_default",with = "quick_xml::serde_helpers::text_content")]
                select: Select,
                #[serde(rename="subskill")]
                subskills : Vec<SubSkill>,
            },
            //$($Passive_extra {$($Passive_extra1 : $Passive_extra2),*},)*
        }

        impl Wave<'_> {
            pub fn execute_skill(&mut self, skill : &Skill,  actor :InstanceIndex, target :InstanceIndex, ) {
                match skill {
                    Skill::None => {},
                    Skill::Generic {  ..} => {self.execute_generic_skill(skill, actor, target)},
                    $(Skill::$Passive => {panic!("No exec on passsive")})*
                    //$(Skill::$Passive_extra {..} => {panic!("No exec on passsive")})*
                    $(Skill::$Special (s) => {s.execute(self,skill,actor,target)})*
                }
                self.cooldown_s(actor,skill);
            }
        }

        pub fn get_type(skill :&Skill)-> SkillType {
            match skill {
                Skill::None => SkillType::None,
                Skill::Generic { cooldown, ..} => return SkillType::Active,
                $(Skill::$Passive => {return SkillType::Passive})*
                //$(Skill::$Passive_extra {..} => {return SkillType::Passive})*
                $(Skill::$Special ($Special {..}) => return $Special::TYPE,)*
            }
        } 

        pub fn get_select(skill :&Skill)-> Select{
            match skill {
                Skill::None => Select::None,
                Skill::Generic {select, ..} => *select,
                $(Skill::$Passive => {return Select::None})*
                //$(Skill::$Passive_extra {..} => {return Select::None})*
                $(Skill::$Special ($Special {..}) => return $Special::SELECT,)*
            }
        }

        pub fn get_cooldown(skill:&Skill) -> u32 {
            match skill {
                Skill::None => 0,
                Skill::Generic { cooldown, ..} => return *cooldown,
                $(Skill::$Passive => {return 0})*
                //$(Skill::$Passive_extra {..} => {return 0})*
                $(Skill::$Special (s) => return s.get_cooldown(),)*
                //$(Skill::$Special ($Special {cooldown,..}) => return *cooldown,)*
            }
        }
    }
}

gen_match!(         
        [
        CounterAttack,
        SharpInstinct,
        IncessantChatter
        ],
        //[Resplendence {turn_meter_ratio : f32}],
        [
        BloodthirstyScythe
        ,BurstingKnowledge
        ,Resplendence
        ,ScorchedSoul      
        ,FireHeal          
        ,Resurrection      
        ,ScytheStrike
        ,EnergyBurst       
        ,TideBigHit        
        ,DeepSeaPower      
        ,CrystalOfLife     
        ,Tricks            
        ,Nightmare         
        ,FissionOfLife     
        ,SoulRing
        ,SoulSurge
        ,SoulSeal
        ,ScarletSlash      
        ,LeavesStorm       
        ,ScarletMultiStrike 
        ,DarknightStrike   
        ,EyeForAnEye       
        ,DarknightArbitrament
        ,SpiritCall
        ,SpiritFountain
        ,Commendation 
        ,CounterattackCommand
        ,ForceOfMercy
        ,SacredLight
        ,LightOfPurifying
        ,BloodlustStrike
        ,BloodthirstyDesire
        ,FishDive
        ,FishGuardian
        ,FishWaterball
        ,CleanOcean
        ,Detach
        ,BasicAttack
        ]
    );


//pub fn get_cooldown(skill: &Skill) ->u32 {
//    return skill.cooldown;
//    //match skill {
//    //    //Liz
//    //    Skill::ScorchedSoul       {cooldown,..} => *cooldown,
//    //    Skill::FireHeal           { cooldown, ..} => *cooldown,
//    //    Skill::Resurrection       { cooldown, .. } => *cooldown,
//    //    //Natalie
//    //    Skill::ScytheStrike       { cooldown,.. } => *cooldown,
//    //    Skill::BloodthirstyScythe { cooldown,.. } => *cooldown,
//    //    Skill::EnergyBurst        { cooldown,.. } => *cooldown,
//    //    //Seth
//    //    Skill::TideBigHit { cooldown,..} => *cooldown,
//    //    Skill::DeepSeaPower { cooldown, ..} => *cooldown,
//    //    Skill::CrystalOfLife { cooldown, ..} => *cooldown,
//    //    //Space
//    //    Skill::Tricks{cooldown,..} => *cooldown,
//    //    Skill::Nightmare { cooldown, ..} => *cooldown,
//    //    Skill::FissionOfLife { cooldown, ..} => *cooldown,
//    //    //Tifya
//    //    Skill::ScarletSlash { cooldown, ..} => *cooldown,
//    //    Skill::LeavesStorm { cooldown, ..} => *cooldown,
//    //    Skill::ScaletMultiStrike { cooldown, ..} => *cooldown,
//    //    //Hazier
//    //    Skill::DarknightStrike { cooldown,..} => *cooldown,
//    //    Skill::EyeForAnEye { cooldown, basic_attack, counterattack_turns: counter_attack_turns, damage_immunity_turns, control_immunity_turns } => *cooldown,
//    //    Skill::DarknightArbitrament { cooldown, ..} => *cooldown,
//    //    //Geeliman
//    //    Skill::BurstingKnowledge { cooldown, ..} => *cooldown,
//    //    //
//    //    Skill::BasicAttack{cooldown,..} => *cooldown,
//    //    Skill::Generic{ cooldown, ..} => *cooldown,
//    //}
//}

// TODO why option and not just empty array?!?!?

fn get_alive_allies<const LEN:usize>(wave : &Wave, actor :  InstanceIndex) -> Option<Vec<InstanceIndex>> {
    let team = wave.get_ally_indices(actor);
    let mut ids = Vec::new();
    for (_index,&target) in team.iter().enumerate() {
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

fn get_alive_enemies<const LEN:usize>( wave :&Wave,actor :InstanceIndex,) -> Option<Vec<InstanceIndex>> {
    let team = wave.get_enemies_indices(actor);
    let mut ids = Vec::new();
    for (_index,&target) in team.iter().enumerate() {
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
    use quick_xml::{de::from_str, se::to_string};

    use super::*;

    #[test]
    fn write_xml() {
        let skill= 
            //typ : SkillType::Active,
            //select: Select::SingleEnemy,
            Skill::ScorchedSoul (ScorchedSoul{
                cooldown : 3,
                attack_damage_ratio : 1.0,
                hp_burning_chance: 0.5,
                hp_burning_turns: 2
            });

        let xml = to_string(&skill).unwrap();
        //panic!("{}",xml);
    }

    #[test]
    fn read_xml() {
        let skill: Vec<Skill>= from_str(
            r#"
            <ScorchedSoul><cooldown>3</cooldown><attack_damage_ratio>1</attack_damage_ratio><hp_burning_chance>0.5</hp_burning_chance><hp_burning_turns>2</hp_burning_turns></ScorchedSoul>
            "#,
            /* 
            <skill>
            <cooldown>3</cooldown>
                <ScorchedSoul>
                    <attack_damage_ratio>1.0</attack_damage_ratio>
                    <hp_burning_chance>0.5</hp_burning_chance>
                    <hp_burning_turns>2</hp_burning_turns>
                </ScorchedSoul>
            </skill>
            */
        )
        .unwrap();

        match skill[0] {
            Skill::ScorchedSoul(ScorchedSoul{attack_damage_ratio,hp_burning_chance, hp_burning_turns ,..}) => {
                assert_eq!(attack_damage_ratio, 1.0);
                assert_eq!(hp_burning_chance, 0.5);
                assert_eq!(hp_burning_turns, 2);
            }
            _ => panic!("Wrong skill type , {:?}", skill),
        }
    }
}