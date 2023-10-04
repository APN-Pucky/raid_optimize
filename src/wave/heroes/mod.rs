pub mod space;
pub mod tifya;
pub mod liz;
pub mod natalie;
pub mod hazier;
pub mod geeliman;
pub mod seth;
pub mod margarita;
pub mod marville;
pub mod dakota;
pub mod maya;
pub mod alahan;

pub trait Execute {
    fn execute(&self, wave : &mut crate::wave::Wave<'_>, skill : &crate::data::skill::Skill, actor : crate::wave::InstanceIndex, target : crate::wave::InstanceIndex);
}

#[cfg(test)]
pub mod tests {
use rand::seq::SliceRandom;
use tokio::sync::mpsc;

use crate::{data::load_heroes, sim::{results::CombinedResult, args::Args, Sim}};

// constant
pub fn test_1_vs_1(name: &str) {
    let mut rng = rand::thread_rng();
    let heroes = load_heroes("data/heroes.xml".to_string());
    let liz = heroes.heroes.iter().find(|h| h.name == name).unwrap();
    let mut args = Args::default();
    args.threads = 1;
    args.iterations = 100;
    args.allies = vec![liz.clone()];
    args.enemies =  vec![liz.clone()];
    let sim = Sim::new(args.clone());
    let (tx, mut rx) = mpsc::unbounded_channel::<CombinedResult>();
    sim.run(tx);
}

pub fn test_5_vs_5(name: &str) {
    let mut rng = rand::thread_rng();
    let heroes = load_heroes("data/heroes.xml".to_string());
    let liz = heroes.heroes.iter().find(|h| h.name == name).unwrap();
    let mut args = Args::default();
    args.threads = 1;
    args.iterations = 100;
    args.allies = vec![liz.clone(),liz.clone(),liz.clone(),liz.clone(),liz.clone()];
    args.enemies =  vec![liz.clone(),liz.clone(),liz.clone(),liz.clone(),liz.clone()];
    let sim = Sim::new(args.clone());
    let (tx, mut rx) = mpsc::unbounded_channel::<CombinedResult>();
    sim.run(tx);
}
}