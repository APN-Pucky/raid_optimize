use crate::data::skill::{Select, Skill, SkillType};
use derive_macro::Cooldown;

use super::{InstanceIndex, Wave};

pub mod agatha;
pub mod alahan;
pub mod asindo;
pub mod ben_austin;
pub mod dakota;
pub mod ellic;
pub mod geeliman;
pub mod guhanna;
pub mod hazier;
pub mod katherine;
pub mod liz;
pub mod margarita;
pub mod marville;
pub mod maya;
pub mod natalie;
pub mod nita;
pub mod nordak;
pub mod paulin;
pub mod phocas;
pub mod seth;
pub mod space;
pub mod tifya;

pub trait Typed {
    const TYPE: SkillType;
    //fn get_type()-> SkillType;
}

pub trait Selector {
    const SELECT: Select;
    //fn get_select()-> Select;
}

pub trait Cooldown {
    fn get_cooldown(&self) -> u32;
}

pub trait Execute {
    fn execute(
        &self,
        wave: &mut crate::wave::Wave<'_>,
        skill: &crate::data::skill::Skill,
        actor: crate::wave::InstanceIndex,
        target: crate::wave::InstanceIndex,
    );
}

pub trait Skilled: Cooldown + Selector + Typed + Execute {}

pub trait PassiveSkill {
    const TYPE: SkillType = SkillType::Passive;
    const SELECT: Select = Select::None;

    fn execute(
        &self,
        _wave: &mut crate::wave::Wave<'_>,
        _skill: &crate::data::skill::Skill,
        _actor: crate::wave::InstanceIndex,
        _target: crate::wave::InstanceIndex,
    ) {
        panic!("Passive skill should not be executed");
    }
}

#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct BasicAttack {
    pub cooldown: u32,
    pub attack_damage_ratio: f32,
}
impl Default for BasicAttack {
    fn default() -> Self {
        BasicAttack {
            cooldown: 0,
            attack_damage_ratio: 1.0,
        }
    }
}
impl BasicAttack {
    pub const TYPE: SkillType = SkillType::Basic;
    pub const SELECT: Select = Select::SingleEnemy;

    pub fn execute(
        &self,
        wave: &mut Wave,
        skill: &Skill,
        actor: InstanceIndex,
        defender: InstanceIndex,
    ) {
        wave.attack_single(
            actor,
            defender,
            wave.get_attack_damage(actor) * self.attack_damage_ratio,
            skill,
        );
    }
}

#[cfg(test)]
pub mod tests {

    use tokio::sync::mpsc;

    use crate::{
        data::load_heroes,
        sim::{args::Args, results::CombinedResult, Sim},
    };

    // constant
    pub fn test_1_vs_1(name: &str) {
        let _rng = rand::thread_rng();
        let heroes = load_heroes("data/heroes.xml".to_string());
        let liz = heroes.heroes.iter().find(|h| h.name == name).unwrap();
        let mut args = Args::default();
        args.threads = 1;
        args.iterations = 10;
        args.allies = vec![liz.clone()];
        args.enemies = vec![liz.clone()];
        let sim = Sim::new(args.clone());
        let (tx, _rx) = mpsc::unbounded_channel::<CombinedResult>();
        sim.run(tx);
    }

    pub fn test_5_vs_5(name: &str) {
        let _rng = rand::thread_rng();
        let heroes = load_heroes("data/heroes.xml".to_string());
        let liz = heroes.heroes.iter().find(|h| h.name == name).unwrap();
        let mut args = Args::default();
        args.threads = 1;
        args.iterations = 10;
        args.allies = vec![
            liz.clone(),
            liz.clone(),
            liz.clone(),
            liz.clone(),
            liz.clone(),
        ];
        args.enemies = vec![
            liz.clone(),
            liz.clone(),
            liz.clone(),
            liz.clone(),
            liz.clone(),
        ];
        let sim = Sim::new(args.clone());
        let (tx, _rx) = mpsc::unbounded_channel::<CombinedResult>();
        sim.run(tx);
    }
}

macro_rules! test_hero {
    ($hero:ident) => {
        #[cfg(test)]
        pub mod tests {
            #[test]
            fn test_1_vs_1() {
                crate::wave::heroes::tests::test_1_vs_1(stringify!($hero));
            }
            #[test]
            fn test_5_vs_5() {
                crate::wave::heroes::tests::test_5_vs_5(stringify!($hero));
            }
        }
    };
}
pub(crate) use test_hero;

macro_rules! test_hero_string {
    ($hero:expr) => {
        #[cfg(test)]
        pub mod tests {
            #[test]
            fn test_1_vs_1() {
                crate::wave::heroes::tests::test_1_vs_1($hero);
            }
            #[test]
            fn test_5_vs_5() {
                crate::wave::heroes::tests::test_5_vs_5($hero);
            }
        }
    };
}
pub(crate) use test_hero_string;
