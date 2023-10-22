use strum_macros::EnumIter;

use crate::wave::heroes::agatha::aristocratic_style::AristocraticStyle;
use crate::wave::heroes::agatha::bloody_spiral::BloodySpiral;
use crate::wave::heroes::alahan::commendation::Commendation;
use crate::wave::heroes::alahan::detach::Detach;
use crate::wave::heroes::alahan::spirit_call::SpiritCall;
use crate::wave::heroes::alahan::spirit_fountain::SpiritFountain;
use crate::wave::heroes::asindo::psychological_torture::PsychologicalTorture;
use crate::wave::heroes::dakota::soul_ring::SoulRing;
use crate::wave::heroes::dakota::soul_seal::SoulSeal;
use crate::wave::heroes::dakota::soul_surge::SoulSurge;
use crate::wave::heroes::geeliman::bursting_knowledge::BurstingKnowledge;
use crate::wave::heroes::guhanna::lunar_recovery::LunarRecovery;
use crate::wave::heroes::guhanna::lunar_shelter::LunarShelter;
use crate::wave::heroes::hazier::bloodlust_strike::BloodlustStrike;
use crate::wave::heroes::hazier::darknight_arbitrament::DarknightArbitrament;
use crate::wave::heroes::hazier::darknight_strike::DarknightStrike;
use crate::wave::heroes::hazier::eye_for_an_eye::EyeForAnEye;
use crate::wave::heroes::liz::fire_heal::FireHeal;
use crate::wave::heroes::liz::resurrection::Resurrection;
use crate::wave::heroes::liz::scorched_soul::ScorchedSoul;
use crate::wave::heroes::margarita::counterattack_command::CounterattackCommand;
use crate::wave::heroes::marville::clean_ocean::CleanOcean;
use crate::wave::heroes::marville::fish_dive::FishDive;
use crate::wave::heroes::marville::fish_guardian::FishGuardian;
use crate::wave::heroes::marville::fish_waterball::FishWaterball;
use crate::wave::heroes::maya::force_of_mercy::ForceOfMercy;
use crate::wave::heroes::maya::light_of_purifying::LightOfPurifying;
use crate::wave::heroes::maya::sacred_light::SacredLight;
use crate::wave::heroes::natalie::bloodthirsty_scythe::BloodthirstyScythe;
use crate::wave::heroes::natalie::bloodthristy_desire::BloodthirstyDesire;
use crate::wave::heroes::natalie::energy_burst::EnergyBurst;
use crate::wave::heroes::natalie::scythe_strike::ScytheStrike;
use crate::wave::heroes::nita::bondage::Bondage;
use crate::wave::heroes::nita::curse::Curse;
use crate::wave::heroes::nita::deep_trap::DeepTrap;
use crate::wave::heroes::phocas::divine_prayer::DivinePrayer;
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
use crate::wave::heroes::BasicAttack;
use crate::wave::heroes::Cooldown;
use crate::wave::heroes::Execute;
use crate::wave::heroes::PassiveSkill;
use crate::wave::heroes::Selector;
use crate::wave::heroes::Typed;
use crate::wave::InstanceIndex;
use crate::wave::Wave;

use super::subskill::SubSkill;

use strum_macros::EnumString;

pub struct NewSkill {
    pub cooldown: u32,
    pub subskills: Vec<SubSkill>,
}

#[derive(Debug, PartialEq, strum_macros::Display, Deserialize, Serialize, Clone, Copy)]
pub enum SkillType {
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

#[derive(
    Default,
    EnumIter,
    Deserialize,
    Serialize,
    strum_macros::Display,
    Debug,
    Clone,
    Eq,
    PartialEq,
    Copy,
)]
pub enum Select {
    #[default]
    None,
    Everyone,
    SingleAlly,
    SingleEnemy,
    AllEnemies,
    AllAllies,
    SingleSelf,
}

pub fn get_selection(wave: &Wave, select: Select, actor: InstanceIndex) -> Vec<InstanceIndex> {
    match select {
        Select::Everyone => {
            // 0..LEN
            wave.get_indices_iter().collect()
        }
        Select::SingleAlly => wave.get_ally_indices(actor),
        Select::SingleEnemy => wave.get_enemies_indices(actor),
        Select::AllEnemies => wave.get_enemies_indices(actor),
        Select::AllAllies => wave.get_ally_indices(actor),
        Select::SingleSelf => {
            vec![actor]
        }
        Select::None => {
            vec![]
        }
    }
}

pub fn is_passive(skill: &Skill) -> bool {
    return get_type(skill) == SkillType::Passive;
}

pub fn is_reducable(skill: &Skill) -> bool {
    match skill {
        Skill::FishDive { .. } => false,
        Skill::DivinePrayer { .. } => false,
        _ => true,
    }
}

pub fn is_basic_attack(skill: &Skill) -> bool {
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

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Generic {
    pub cooldown: u32,
    pub name: String,
    #[serde(
        default = "select_default",
        with = "quick_xml::serde_helpers::text_content"
    )]
    pub select: Select,
    #[serde(rename = "subskill")]
    pub subskills: Vec<SubSkill>,
}
impl Default for Generic {
    fn default() -> Self {
        Generic {
            cooldown: 0,
            name: "Generic".to_string(),
            select: Select::None,
            subskills: vec![],
        }
    }
}

macro_rules! gen_match {
    ( [$($Passive:ident),*],
    //[$($Passive_extra:ident {$($Passive_extra1:ident : $Passive_extra2:ident),*}),*],
      [$($Special:ident),*] ) => {
        #[derive(EnumString, EnumIter, Debug, PartialEq,strum_macros::Display, Deserialize, Serialize, Clone )]
        pub enum Skill {
            None,
            Generic ( Generic),
            $($Special ($Special),)*
            $($Passive,)*
            //$($Passive_extra {$($Passive_extra1 : $Passive_extra2),*},)*
        }

        impl Wave<'_> {
            pub fn execute_skill(&mut self, skill : &Skill,  actor :InstanceIndex, target :InstanceIndex, ) {
                self.cooldown_s(actor,skill); // Early cooldown so no reuse
                match skill {
                    Skill::None => {},
                    Skill::Generic (Generic{  ..}) => {self.execute_generic_skill(skill, actor, target)},
                    $(Skill::$Passive => {panic!("No exec on passsive")})*
                    //$(Skill::$Passive_extra {..} => {panic!("No exec on passsive")})*
                    $(Skill::$Special (s) => {s.execute(self,skill,actor,target)})*
                }
            }
        }

        pub fn get_type(skill :&Skill)-> SkillType {
            match skill {
                Skill::None => SkillType::None,
                Skill::Generic (Generic{ cooldown: _, ..}) => return SkillType::Active,
                $(Skill::$Passive => {return SkillType::Passive})*
                //$(Skill::$Passive_extra {..} => {return SkillType::Passive})*
                $(Skill::$Special ($Special {..}) => return $Special::TYPE,)*
            }
        }

        pub fn get_select(skill :&Skill)-> Select{
            match skill {
                Skill::None => Select::None,
                Skill::Generic (Generic{select, ..}) => *select,
                $(Skill::$Passive => {return Select::None})*
                //$(Skill::$Passive_extra {..} => {return Select::None})*
                $(Skill::$Special ($Special {..}) => return $Special::SELECT,)*
            }
        }

        pub fn get_cooldown(skill:&Skill) -> u32 {
            match skill {
                Skill::None => 0,
                Skill::Generic (Generic{ cooldown, ..}) => return *cooldown,
                $(Skill::$Passive => {return 0})*
                //$(Skill::$Passive_extra {..} => {return 0})*
                $(Skill::$Special (s) => return s.get_cooldown(),)*
                //$(Skill::$Special ($Special {cooldown,..}) => return *cooldown,)*
            }
        }
    }
}

gen_match!(
    [Counterattack, IncessantChatter, SharpInstinct],
    //[Resplendence {turn_meter_ratio : f32}],
    [
        AristocraticStyle,
        BasicAttack,
        BloodthirstyScythe,
        BurstingKnowledge,
        BloodlustStrike,
        BloodthirstyDesire,
        BloodySpiral,
        Bondage,
        Curse,
        CleanOcean,
        CrystalOfLife,
        Commendation,
        CounterattackCommand,
        DarknightStrike,
        DarknightArbitrament,
        DeepSeaPower,
        DeepTrap,
        Detach,
        DivinePrayer,
        EnergyBurst,
        EyeForAnEye,
        FissionOfLife,
        FireHeal,
        FishDive,
        FishGuardian,
        FishWaterball,
        ForceOfMercy,
        LeavesStorm,
        LunarShelter,
        LunarRecovery,
        LightOfPurifying,
        Nightmare,
        PsychologicalTorture,
        Resplendence,
        Resurrection,
        ScorchedSoul,
        ScytheStrike,
        SacredLight,
        SoulRing,
        SoulSurge,
        SoulSeal,
        ScarletSlash,
        ScarletMultiStrike,
        SpiritCall,
        SpiritFountain,
        TideBigHit,
        Tricks
    ]
);

#[cfg(test)]
mod tests {
    use quick_xml::{de::from_str, se::to_string};

    use super::*;

    #[test]
    fn write_xml() {
        let skill = Skill::ScorchedSoul(ScorchedSoul {
            cooldown: 3,
            attack_damage_ratio: 1.0,
            hp_burning_chance: 0.5,
            hp_burning_turns: 2,
        });

        let _xml = to_string(&skill).unwrap();
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
            Skill::ScorchedSoul(ScorchedSoul {
                attack_damage_ratio,
                hp_burning_chance,
                hp_burning_turns,
                ..
            }) => {
                assert_eq!(attack_damage_ratio, 1.0);
                assert_eq!(hp_burning_chance, 0.5);
                assert_eq!(hp_burning_turns, 2);
            }
            _ => panic!("Wrong skill type , {:?}", skill),
        }
    }
}
